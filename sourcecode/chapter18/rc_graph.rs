use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashSet;

struct Node {
    datum: &'static str,				// 数据
    edges: Vec<Rc<RefCell<Node>>>,		// 边定义 - 也就是邻接矩阵
}

impl Node {
    fn new(datum: &'static str) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            datum: datum,
            edges: Vec::new(),
        }))
    }

    fn traverse<F>(&self, f: &F, seen: &mut HashSet<&'static str>) // 此处seen起到了一个栈的作用
        where F: Fn(&'static str)
    {
        if seen.contains(&self.datum) {	// 如果已经在栈中，则直接返回
            return;
        }
        f(self.datum);
        seen.insert(self.datum);
        for n in &self.edges {
            n.borrow().traverse(f, seen);	// 递归遍历所有的边，深度优先
        }
    }

    fn first(&self) -> Rc<RefCell<Node>> {
        self.edges[0].clone()
    }
}

fn foo(node: &Node) {
    println!("foo: {}", node.datum);
}

fn init() -> Rc<RefCell<Node>> {	// 初始化图
    let root = Node::new("A");

    let b = Node::new("B");
    let c = Node::new("C");
    let d = Node::new("D");
    let e = Node::new("E");
    let f = Node::new("F");

    {
        let mut mut_root = root.borrow_mut();
        mut_root.edges.push(b.clone());		// 将b,c,d节点陆续加入图中
        mut_root.edges.push(c.clone());
        mut_root.edges.push(d.clone());

        let mut mut_c = c.borrow_mut();
        mut_c.edges.push(e.clone());		// 对节点c，加入边ce和cf，ca
        mut_c.edges.push(f.clone());
        mut_c.edges.push(root.clone());
    }

    root
}

pub fn main() {
    let g = init();
    let g = g.borrow();
    g.traverse(&|d| println!("{}", d), &mut HashSet::new());
    let f = g.first();
    foo(&*f.borrow());
}