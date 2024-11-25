use std::sync::Arc;

use serde::Serialize;
use serde::Deserialize;
use serde;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct A {
    f1: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct B {
    f1: String,
}

#[typetag::serde(tag = "type")]
trait MyTrait {
    fn str(&self) -> String;
}

#[typetag::serde]
impl MyTrait for A {
    fn str(&self) -> String {
        "This is A".to_string()
    }
}

#[typetag::serde]
impl MyTrait for B {
    fn str(&self) -> String {
        "This is B".to_string()
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct ArcContainer {
    obj: Arc<dyn MyTrait>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
struct Container {
    aaa: ArcContainer,
}

impl Default for ArcContainer {
    fn default() -> Self {
        Self{obj: Arc::new(A{f1: "A".to_string()})}
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct Container2 {
    #[serde(default)]
    container: Container,
}

fn main() -> serde_json::Result<()> {
    let a = A{f1: "A".to_string()};
    let b = B{f1: "B".to_string()};
    println!("Hello, world!:\nA: {}\nB: {}", a.str(), b.str());
    let a_trait = &a as &dyn MyTrait;
    let a_trait_serialized = serde_json::to_string(a_trait)?;
    println!("{a_trait_serialized}");
    let b_trait = &b as &dyn MyTrait;
    let b_trait_serialized = serde_json::to_string(b_trait)?;
    println!("{b_trait_serialized}");

    let de: Box<dyn MyTrait> = serde_json::from_str(&b_trait_serialized)?;
    println!("{}", de.str());

    let cont = Container{..Default::default()};
    let cont_serialized = serde_json::to_string(&cont)?;
    println!("{cont_serialized}");
    let cont2 = serde_json::from_str::<Container>(&cont_serialized)?;
    println!("{}", cont2.aaa.obj.str());

    Ok(())
}
