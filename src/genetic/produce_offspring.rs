use super::build_node::Node;
use rand::Rng;

pub fn produce_offspring(node: &mut Node, parent_a: &Node, parent_b: &Node) {
    let mut rng = rand::thread_rng();
    node.stats.iter_mut().for_each(|s| {
        let mut choose_parent = parent_a;

        let roll = rng.gen::<f32>();
        if roll > 0.5 {
            choose_parent = parent_b;
        }

        s.value = choose_parent
            .stats
            .iter()
            .find(|f| f.stat == s.stat)
            .expect("Should always find node")
            .value;
    });
}
