
use pest::{Parser};
use pest::iterators::{Pair, Pairs};

#[derive(Parser)]
#[grammar = "lambda.pest"]
pub struct LambdaParser;

#[derive(Debug)]
enum NodeType {
  Application,
  Function,
  Name
}

#[derive(Debug)]
struct Node {
  node_type: NodeType,
  value: Option<String>,
  children: Vec<Node>
}
/*
impl Node {
  fn new(pairs: Pair<Rule>) -> Node {
    
    match pairs.as_rule() {
      Rule::expression => Node::expression(pairs.into_inner())
    }
  }

  fn expression(pairs: Pairs<Rule>) -> Node {

  }
  
}
*/

fn to_pairs(source: &str) -> Result<Pairs<Rule>, pest::error::Error<Rule>>  {
  LambdaParser::parse(Rule::input, source)
}


fn pairs_to_ast(pairs: Pairs<Rule>) -> Vec<Node> {
  pairs.map(|p| pair_to_node(p)).collect()

}

fn pair_to_node(pair: Pair<Rule>) -> Node {
  match pair.as_rule() {
    Rule::application => Node{node_type: NodeType::Application, value: None, children: vec![]},
    _ => Node{node_type: NodeType::Name, value: None, children: vec![]}
  }
}

fn application_to_node(pair: Pair<Rule>) -> Node {
  let children : Vec<Pair<Rule>> = pair.into_inner().collect();
  Node{
    node_type: NodeType::Application
  , value: None
  , children: vec![pair_to_node(children[0].clone()), pair_to_node(children[1].clone())]
  }
}

fn pretty_print(pairs: &mut Pairs<Rule>) -> String {
  let lines: Vec<String> = pairs.map(|pair| 
    {format_pair(&pair, 0, true)}
  ).collect::<Vec<String>>();
  lines.join("\n")
}


fn format_pair(pair: &Pair<Rule>, indent_level: usize, is_newline: bool) -> String {
  let indent = if is_newline {
      "  ".repeat(indent_level)
  } else {
      "".to_string()
  };

  let children: Vec<_> = pair.clone().into_inner().collect();
  let len = children.len();
  let children: Vec<_> = children.into_iter().map(|pair| {
      format_pair(&pair, if len > 1 { indent_level + 1 } else { indent_level }, len > 1)
  }).collect();

  let dash = if is_newline {
      "- "
  } else {
      ""
  };

  match len {
      0 => format!("{}{}{:?}: {:?}", indent, dash, pair.as_rule(), pair.as_span().as_str()),
      1 => format!("{}{}{:?} > {}", indent, dash, pair.as_rule(), children[0]),
      _ => format!("{}{}{:?}\n{}", indent, dash, pair.as_rule(), children.join("\n"))
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test1() {
    let input = format!("(λx. xy)λxy.x");
    let mut pairs = to_pairs(&input).unwrap();
    println!("{}", pretty_print(&mut pairs));
    let nodes = pairs_to_ast(pairs);
    println!("");
    println!("{:?}", nodes)
  }
}
