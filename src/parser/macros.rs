use crate::parser::{Attribute, Edge, EdgeTy, GraphAttributes, Id, Node, Port, Subgraph,Graph};

#[macro_export]
macro_rules! port {
    () => {None};
    ($id:expr, $str:expr) => {Some(Port($id,$str))}
}



#[macro_export]
macro_rules! id {
    () => { Id::Anonymous("".to_string()) };
    (html$e:expr) => { Id::Html(format!("{}",$e))};
    (esc$e:expr) => { Id::Escaped(format!("{}",$e))};
    ($e:expr) => { Id::Plain(format!("{}",$e))};
}

#[macro_export]
macro_rules! a_attr {
    ($ik:ident $k:expr,$iv:ident $v:expr) => {Attribute::Arbitrary(id!($k),id!($iv $v))};
    ($ik:ident $k:expr,$v:expr) => {Attribute::Arbitrary(id!($ik $k),id!($v))};
    ($k:expr, $iv:ident $v:expr) => {Attribute::Arbitrary(id!($k),id!($iv $v))};
    ($k:expr,$v:expr) => {Attribute::Arbitrary(id!($k),id!($v))}
}
#[macro_export]
macro_rules! stmt {
    ($k:expr) => {Stmt::from($k)};
}

#[macro_export]
macro_rules! subgraph {
    () => {Subgraph{id:id!(),stmts:vec![]}};
    ($id:expr) => {Subgraph{id:id!($id),stmts:vec![]}};
    ($i:ident $id:expr) => {Subgraph{id:id!($i$id),stmts:vec![]}};
    ($id:expr, $stmts:expr) => {Subgraph{id:id!($id),stmts:$stmts}};
    ($i:ident $id:expr, $stmts:expr) => {Subgraph{id:id!($i$id),stmts:$stmts}};
    ($i:ident $id:expr; $($stmts:expr),+ ) => {{
        let mut stmts_vec = Vec::new();
        $( stmts_vec.push(stmt!($stmts)) ; )+
        Subgraph{id:id!($i$id),stmts:stmts_vec}
    }};
    ($id:expr; $($stmts:expr),+ ) => {{
        let mut stmts_vec = Vec::new();
        $( stmts_vec.push(stmt!($stmts)) ; )+
        Subgraph{id:id!($id),stmts:stmts_vec}
    }};

}


#[macro_export]
macro_rules! node {
    () => {Node::new(NodeId(id!(), None), vec![])};
    ($i:ident $id:expr) => {Node::new(NodeId(id!($i$id), None), vec![])};
    ($id:expr) => {Node::new(NodeId(id!($id), None), vec![])};
    ($i:ident $id:expr; $($attr:expr),+ ) => {{
        let mut attrs = Vec::new();
        $( attrs.push($attr) ; )+
        Node::new(NodeId(id!($i$id), None), attrs)
    }};
     ($i:ident $id:expr, $attrs:expr  ) => {
        Node::new(NodeId(id!($i$id), None), $attrs)
    };
    ($id:expr, $attrs:expr  ) => {
        Node::new(NodeId(id!($id), None), $attrs)
    };
    ( $id:expr; $($attr:expr),+ ) => {{
        let mut attrs = Vec::new();
        $( attrs.push($attr) ; )+
        Node::new(NodeId(id!( $id), None), attrs)
    }};
    ($i:ident $id:expr => $p:expr, $attrs:expr  ) => {
        Node::new(NodeId(id!($i$id), $p), $attrs)
    };
    ($i:ident $id:expr => $p:expr; $($attr:expr),+ ) => {{
        let mut attrs = Vec::new();
         $( attrs.push($attr) ; )+
        Node::new(NodeId(id!($i$id), $p), attrs)
    }};
    ( $id:expr => $p:expr, $attrs:expr  ) => {
        Node::new(NodeId(id!($id), $p), $attrs)
    };
    ( $id:expr => $p:expr; $($attr:expr),+ ) => {{
        let mut attrs = Vec::new();
        $( attrs.push($attr) ; )+
        Node::new(NodeId(id!($id), $p), attrs)
    }};
}

#[macro_export]
macro_rules! edge {
    ($l:expr => $r:expr) => {
        Edge{ ty: EdgeTy::Pair(Vertex::from($l),Vertex::from($r)), attributes: vec![] }
    };
    ($l:expr => $r:expr $(=> $nexts:expr)+) => {{
         let mut edges_vec = vec![Vertex::from($l),Vertex::from($r)];
         $( edges_vec.push(Vertex::from($nexts)) ; )+

        Edge{ ty: EdgeTy::Chain(edges_vec), attributes: vec![] }
    }};

    ($l:expr => $r:expr, $attrs:expr) => {
        Edge{ ty: EdgeTy::Pair(Vertex::from($l),Vertex::from($r)), attributes: $attrs };
    };
    ($l:expr => $r:expr; $($attrs:expr),+) => {{
         let mut attrs_vec = Vec::new();
        $( attrs_vec.push($attrs) ; )+
        Edge{ ty: EdgeTy::Pair(Vertex::from($l),Vertex::from($r)), attributes: attrs_vec }
    }};
    ($l:expr => $r:expr $(=> $nexts:expr)+; $($attrs:expr),+) => {{
         let mut attrs_vec = Vec::new();
         $( attrs_vec.push($attrs) ; )+

         let mut edges_vec = vec![Vertex::from($l),Vertex::from($r)];
         $( edges_vec.push(Vertex::from($nexts)) ; )+

        Edge{ ty: EdgeTy::Chain(edges_vec), attributes: attrs_vec }
    }};
    ($l:expr => $r:expr $(=> $nexts:expr)+ , $attrs:expr) => {{

         let mut edges_vec = vec![Vertex::from($l),Vertex::from($r)]
         $( edges_vec.push(Vertex::from($nexts)) ; )+

        Edge{ ty: EdgeTy::Chain(edges_vec), attributes: $attrs }
    }};
}

#[macro_export]
macro_rules! graph {
    (strict $id:expr) => {
        Graph::Graph { id: $id, strict: true, stmts: vec![] }
    };
    ($id:expr) => {
        Graph::Graph { id: $id, strict: false, stmts: vec![] }
    };
    (strict di $id:expr) => {
        Graph::DiGraph { id: id!($id), strict: true, stmts: vec![] }
    };
    (di $id:expr) => {
        Graph::DiGraph { id: id!($id), strict: false, stmts: vec![] }
    };
    (strict $id:expr, $stmts:expr) => {
        Graph::Graph { id: $id, strict: true, stmts: $stmts }
    };
    ($id:expr, $stmts:expr) => {
        Graph::Graph { id: $id, strict: false, stmts: $stmts }
    };
    (strict di $id:expr, $stmts:expr) => {
        Graph::DiGraph { id: $id, strict: true, stmts: $stmts }
    };
    (di $id:expr, $stmts:expr) => {
        Graph::DiGraph { id: $id, strict: false, stmts: $stmts }
    };


    (strict $id:expr; $($stmts:expr),+) => {{
         let mut stmts = vec![];
         $( stmts.push(stmt!($stmts)) ; )+
        Graph::Graph { id: $id, strict: true, stmts: stmts }
    }};
    ($id:expr; $($stmts:expr),+) => {{
         let mut stmts = vec![];
         $( stmts.push(stmt!($stmts)) ; )+
        Graph::Graph { id: $id, strict: false, stmts: stmts }
    }};
    (strict di $id:expr; $($stmts:expr),+) => {{
         let mut stmts = vec![];
          $( stmts.push(stmt!($stmts)) ; )+
        Graph::DiGraph { id: $id, strict: true, stmts: stmts }
    }};
    (di $id:expr; $($stmts:expr),+) => {{
         let mut stmts = vec![];
          $( stmts.push(stmt!($stmts)) ; )+
        Graph::DiGraph { id: $id, strict: false, stmts: stmts }
    }};

}


mod tests {
    use crate::parser::{Attribute, Edge, EdgeTy, Graph, GraphAttributes, Id, Node, NodeId, Port, Stmt, Subgraph, Vertex};
    use crate::parser::Id::Anonymous;

    #[test]
    fn graph_test(){
        assert_eq!(
            graph!(strict di id!("abc")),
            Graph::Graph {id:id!("abc"),strict:true,stmts:vec![]}
        ) ;
    }
    #[test]
    fn edge_test(){
        assert_eq!(
            edge!(node!("1") => node!("2")),
            Edge{ ty: EdgeTy::Pair(Vertex::N(node!("1")),Vertex::N(node!("2"))), attributes: vec![] }
        ) ;
        assert_eq!(
            edge!(node!("1") => node!("2") => subgraph!("a")),
            Edge{ ty: EdgeTy::Chain(vec![Vertex::N(node!("1")),Vertex::N(node!("2")),Vertex::S(subgraph!("a"))]), attributes: vec![] }
        ) ;
        assert_eq!(
            edge!(node!("1") => node!("2"), vec![a_attr!("a","b")]),
            Edge{ ty: EdgeTy::Pair(Vertex::N(node!("1")),Vertex::N(node!("2"))), attributes: vec![a_attr!("a","b")] }
        );
        assert_eq!(
            edge!(node!("1") => node!("2"); a_attr!("a","b")),
            Edge{ ty: EdgeTy::Pair(Vertex::N(node!("1")),Vertex::N(node!("2"))), attributes: vec![a_attr!("a","b")] }
        );
    }
    #[test]
    fn stmt_test(){
        assert_eq!(stmt!(node!()),Stmt::Node(Node::new(NodeId(id!(), None), vec![])));
    }
    #[test]
    fn subgraph_test(){
        assert_eq!(subgraph!(),Subgraph{ id: Anonymous("".to_string()), stmts: vec![] });
        assert_eq!(subgraph!("abc";node!()),
                   Subgraph{ id: Id::Plain("abc".to_string()),
                       stmts: vec![stmt!(node!())] });
    }

    #[test]
    fn node_test() {
        assert_eq!(node!(), Node::new(NodeId(id!(), None), vec![]));
        assert_eq!(node!(html "abc"; a_attr!("a","a")),
                   Node::new(NodeId(id!(html "abc"), None),
                             vec![a_attr!("a","a")]));
        assert_eq!(node!(html "abc" => port!(); a_attr!("a","a")),
                   Node::new(NodeId(id!(html "abc"), None),
                             vec![a_attr!("a","a")]));
        assert_eq!(node!("abc" ; a_attr!("a","a"),a_attr!("a","a")),
                   Node::new(NodeId(id!( "abc"), None),
                             vec![a_attr!("a","a"), a_attr!("a","a")]))
    }

    #[test]
    fn attr_test() {
        assert_eq!(a_attr!("a","1"), Attribute::Arbitrary(id!("a"), id!("1")));
        assert_eq!(a_attr!(html "a","1"), Attribute::Arbitrary(id!(html "a"), id!("1")))
    }

    #[test]
    fn id_test() {
        assert_eq!(id!(), Id::Anonymous("".to_string()));
        assert_eq!(id!(html "<<abc>>"), Id::Html("<<abc>>".to_string()));
        assert_eq!(id!("abc"), Id::Plain("abc".to_string()));
        assert_eq!(id!(esc "\"ab\\\"c\""), Id::Escaped("\"ab\\\"c\"".to_string()));
    }
}