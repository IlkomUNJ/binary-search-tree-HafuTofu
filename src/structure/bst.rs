use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub type BstNodeLink = Rc<RefCell<BstNode>>;
pub type WeakBstNodeLink = Weak<RefCell<BstNode>>;

//this package implement BST wrapper
#[derive(Debug, Clone)]
pub struct BstNode {
    pub key: Option<i32>,
    pub parent: Option<WeakBstNodeLink>,
    pub left: Option<BstNodeLink>,
    pub right: Option<BstNodeLink>,
}

impl BstNode {
    //private interface
    fn new(key: i32) -> Self {
        BstNode {
            key: Some(key),
            left: None,
            right: None,
            parent: None,
        }
    }

    pub fn new_bst_nodelink(value: i32) -> BstNodeLink {
        let currentnode = BstNode::new(value);
        let currentlink = Rc::new(RefCell::new(currentnode));
        currentlink
    }

    /**
     * Get a copy of node link
     */
    pub fn get_bst_nodelink_copy(&self) -> BstNodeLink {
        Rc::new(RefCell::new(self.clone()))
    }

    fn downgrade(node: &BstNodeLink) -> WeakBstNodeLink {
        Rc::<RefCell<BstNode>>::downgrade(node)
    }

    //private interface
    fn new_with_parent(parent: &BstNodeLink, value: i32) -> BstNodeLink {
        let mut currentnode = BstNode::new(value);
        //currentnode.add_parent(Rc::<RefCell<BstNode>>::downgrade(parent));
        currentnode.parent = Some(BstNode::downgrade(parent));
        let currentlink = Rc::new(RefCell::new(currentnode));
        currentlink
    }

    //add new left child, set the parent to current_node_link
    pub fn add_left_child(&mut self, current_node_link: &BstNodeLink, value: i32) {
        let new_node = BstNode::new_with_parent(current_node_link, value);
        self.left = Some(new_node);
    }

    //add new left child, set the parent to current_node_link
    pub fn add_right_child(&mut self, current_node_link: &BstNodeLink, value: i32) {
        let new_node = BstNode::new_with_parent(current_node_link, value);
        self.right = Some(new_node);
    }

    //search the current tree which node fit the value
    pub fn tree_search(&self, value: &i32) -> Option<BstNodeLink> {
        if let Some(key) = self.key {
            if key == *value {
                return Some(self.get_bst_nodelink_copy());
            }
            if *value < key && self.left.is_some() {
                return self.left.as_ref().unwrap().borrow().tree_search(value);
            } else if self.right.is_some() {
                return self.right.as_ref().unwrap().borrow().tree_search(value);
            }
        }
        //default if current node is NIL
        None
    }

    /**seek minimum by recurs
     * in BST minimum always on the left
     */
    pub fn minimum(&self) -> BstNodeLink {
        if self.key.is_some() {
            if let Some(left_node) = &self.left {
                return left_node.borrow().minimum();
            }
        }
        self.get_bst_nodelink_copy()
    }

    pub fn maximum(&self) -> BstNodeLink {
        if self.key.is_some() {
            if let Some(right_node) = &self.right {
                return right_node.borrow().maximum();
            }
        }
        self.get_bst_nodelink_copy()
    }

    /**
     * Return the root of a node, return self if not exist
     */
    pub fn get_root(node: &BstNodeLink) -> BstNodeLink {
        let parent = BstNode::upgrade_weak_to_strong(node.borrow().parent.clone());
        if parent.is_none() {
            return node.clone();
        }
        return BstNode::get_root(&parent.unwrap());
    }

    /**
     * NOTE: Buggy from pull request
     * Find node successor according to the book
     * Should return None, if x_node is the highest key in the tree
     */
    pub fn tree_successor(x_node: &BstNodeLink) -> Option<BstNodeLink> {
        // directly check if the node has a right child, otherwise go to the next block
        if let Some(right_node) = &x_node.borrow().right {
            return Some(right_node.borrow().minimum());
        }
        // empty right child case
        else {
            let mut x_node = x_node;
            let mut y_node = BstNode::upgrade_weak_to_strong(x_node.borrow().parent.clone());
            let mut temp: BstNodeLink;

            while let Some(ref exist) = y_node {
                if let Some(ref left_child) = exist.borrow().left {
                    if BstNode::is_node_match(left_child, x_node) {
                        return Some(exist.clone());
                    }
                }

                temp = y_node.unwrap();
                x_node = &temp;
                y_node = BstNode::upgrade_weak_to_strong(temp.borrow().parent.clone());
            }

            None
        }
    }

    pub fn tree_insert(currlink: &BstNodeLink, value: i32) {
        if currlink.borrow().key.is_none() {
            currlink.borrow_mut().key = Some(value);
        } else if value < currlink.borrow().key.unwrap() {
            if currlink.borrow().left.is_none() {
                currlink.borrow_mut().add_left_child(&currlink, value);
            } else {
                BstNode::tree_insert(&currlink.borrow_mut().clone().left.as_ref().unwrap(), value);
            }
        } else if value > currlink.borrow().key.unwrap() {
            if currlink.borrow().right.is_none() {
                currlink.borrow_mut().add_right_child(&currlink, value);
            } else {
                BstNode::tree_insert(
                    &currlink.borrow_mut().clone().right.as_ref().unwrap(),
                    value,
                );
            }
        }
    }

    pub fn transplant(root: &mut BstNodeLink, u: &BstNodeLink, v: &Option<BstNodeLink>) {
        let upar = u.borrow().parent.clone();
        if upar.is_none() {
            root.clone().borrow_mut().key = v.clone().unwrap().borrow().key.clone();
            if !(BstNode::tree_successor(&root).unwrap().borrow().key.unwrap() == root.clone().borrow().right.as_ref().unwrap().borrow().key.unwrap()) {
                root.clone().borrow_mut().right.as_ref().unwrap().borrow_mut().parent = Some(Rc::downgrade(&root.clone()));
            } else {
                root.clone().borrow_mut().right = v.clone().unwrap().borrow().right.clone();
            }
        } else {
            let upars = BstNode::upgrade_weak_to_strong(upar.clone()).unwrap();
            let mut parmut = upars.borrow_mut();
            if let Some(left) = parmut.left.clone() {
                if BstNode::is_node_match(&left, &u.clone()) {
                    parmut.left = v.clone();
                } else {
                    parmut.right = v.clone();
                }
            } else {
                parmut.right = v.clone();
            } if let Some(v_node) = v.clone() {
                if let Some(upar2) = u.borrow().parent.clone() {
                    v_node.clone().borrow_mut().parent = Some(upar2.clone());
                }
            }
        }
    }

    pub fn tree_delete(root: &mut BstNodeLink, z: &BstNodeLink) {
        let z_left = z.borrow_mut().left.clone();
        let z_right = z.borrow_mut().right.clone();

        if z_left.clone().is_none() {
            BstNode::transplant(root, &z, &z_right.clone());
        } else if z_right.clone().is_none() {
            BstNode::transplant(root, &z, &z_left.clone());
        } else {
            let z_right_node = z_right.clone().unwrap();
            let successor = BstNode::tree_successor(&z);

            if let Some(y) = &successor.clone() {
                if !(BstNode::is_node_match(&y, &z_right_node)) {
                    let successor_right = y.borrow().right.clone();
                    BstNode::transplant(root, &y.clone(), &successor_right);
                    y.as_ref().borrow_mut().right = Some(z_right_node.clone());
                    y
                        .borrow_mut()
                        .right
                        .as_ref()
                        .unwrap()
                        .borrow_mut()
                        .parent = Some(BstNode::downgrade(&y));
                }
                let z_left_for_assign = BstNode::new_bst_nodelink(z.borrow().left.as_ref().unwrap().borrow().key.unwrap());
                z_left_for_assign.borrow_mut().parent = Some(BstNode::downgrade(&y));
                z_left_for_assign.borrow_mut().left = z.borrow().left.clone().unwrap().borrow().left.clone();
                z_left_for_assign.borrow_mut().right = z.borrow().left.clone().unwrap().borrow().right.clone();
                y.clone().borrow_mut().left = Some(z_left_for_assign);
                BstNode::transplant(root, z, &Some(y.clone()));
                y
                    .clone()
                    .borrow_mut()
                    .left
                    .as_ref()
                    .unwrap()
                    .borrow_mut()
                    .parent = Some(BstNode::downgrade(&y.clone()));
            }
        }
    }

    /**
     * Alternate simpler version of tree_successor that made use of is_nil checking
     */
    #[allow(dead_code)]
    pub fn tree_successor_simpler(x_node: &BstNodeLink) -> Option<BstNodeLink> {
        //create a shadow of x_node so it can mutate
        let mut x_node = x_node;
        let right_node = &x_node.borrow().right.clone();
        if BstNode::is_nil(right_node) != true {
            return Some(right_node.clone().unwrap().borrow().minimum());
        }

        let mut y_node = BstNode::upgrade_weak_to_strong(x_node.borrow().parent.clone());
        let y_node_right = &y_node.clone().unwrap().borrow().right.clone();
        let mut y_node2: Rc<RefCell<BstNode>>;
        while BstNode::is_nil(&y_node)
            && BstNode::is_node_match_option(Some(x_node.clone()), y_node_right.clone())
        {
            y_node2 = y_node.clone().unwrap();
            x_node = &y_node2;
            let y_parent = y_node.clone().unwrap().borrow().parent.clone().unwrap();
            y_node = BstNode::upgrade_weak_to_strong(Some(y_parent));
        }

        //in case our sucessor traversal yield root, means self is the highest key
        if BstNode::is_node_match_option(y_node.clone(), Some(BstNode::get_root(&x_node))) {
            return None;
        }

        //default return self / x_node
        return Some(y_node.clone().unwrap());
    }

    /**
     * private function return true if node doesn't has parent nor children nor key
     */
    pub fn is_nil(node: &Option<BstNodeLink>) -> bool {
        match node {
            None => true,
            Some(x) => {
                if x.borrow().parent.is_none()
                    || x.borrow().left.is_none()
                    || x.borrow().right.is_none()
                {
                    return true;
                }
                return false;
            }
        }
    }

    //helper function to compare both nodelink
    fn is_node_match_option(node1: Option<BstNodeLink>, node2: Option<BstNodeLink>) -> bool {
        if node1.is_none() && node2.is_none() {
            return true;
        }
        if let Some(node1v) = node1 {
            return node2.is_some_and(|x: BstNodeLink| x.borrow().key == node1v.borrow().key);
        }
        return false;
    }

    fn is_node_match(anode: &BstNodeLink, bnode: &BstNodeLink) -> bool {
        if anode.borrow().key == bnode.borrow().key {
            return true;
        }
        return false;
    }

    /**
     * As the name implied, used to upgrade parent node to strong nodelink
     */
    pub fn upgrade_weak_to_strong(node: Option<WeakBstNodeLink>) -> Option<BstNodeLink> {
        match node {
            None => None,
            Some(x) => Some(x.upgrade().unwrap()),
        }
    }
}
