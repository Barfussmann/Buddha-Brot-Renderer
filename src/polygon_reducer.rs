use super::polygon::*;
use super::triangulation::*;
use glam::DVec2 as Vec2;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy)]
pub struct Node {
    value: f64,
    pos: Vec2,
    prev: usize,
    this: usize,
    next: usize,
}
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
impl Eq for Node {}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

pub struct PolygonReducer<T: RemoveOrder> {
    nodes: Vec<Node>,
    remove_order_heap: BinaryHeap<Reverse<Node>>,
    len: usize,
    _marker: PhantomData<T>,
}

impl<T: RemoveOrder> PolygonReducer<T> {
    pub fn new(polygon: &Polygon) -> Self {
        let len = polygon.len();
        let mut reducer = Self {
            nodes: polygon
                .get_points()
                .iter()
                .enumerate()
                .map(|(i, p)| Node {
                    value: f64::MAX,
                    pos: *p,
                    next: (i + 1) % len,
                    prev: (i + len - 1) % len,
                    this: i,
                })
                .collect(),
            remove_order_heap: BinaryHeap::with_capacity(len),
            len,
            _marker: PhantomData,
        };
        reducer.init_node_values();
        reducer
    }
    fn get_next(&self, node: &Node) -> &Node {
        &self.nodes[node.next]
    }
    fn get_new_reference(&self, node: &Node) -> &Node {
        assert!(
            self.is_still_valid(node),
            "Can't get new refence to invalid Node"
        );
        &self.get_next(self.get_prev(node))
    }
    fn get_prev(&self, node: &Node) -> &Node {
        &self.nodes[node.prev]
    }
    fn is_still_valid(&self, node: &Node) -> bool {
        let node_to_compare_to = self.nodes[node.this];
        node_to_compare_to.value == node.value &&
        self.get_next(node).prev == self.get_prev(node).next
    }
    fn update_value(&mut self, index: usize) {
        let node = self.nodes[index];
        let this = node.pos;
        let prev = self.get_prev(&node).pos;
        let next = self.get_next(&node).pos;
        if let Some(value) = T::get_value(prev, this, next) {
            self.nodes[index].value = value;
            self.remove_order_heap.push(Reverse(self.nodes[index]));
        };
    }
    fn init_node_values(&mut self) {
        for i in 0..self.nodes.len() {
            self.update_value(i);
        }
    }
    fn get_next_valid_node(&self, node: &Node) -> Option<&Node> {
        if self.is_empty() {
            return None;
        }
        if self.is_still_valid(node) {
            return Some(self.get_new_reference(node));
        }

        let mut next_node = self.get_next(node);
        loop {
            if self.is_still_valid(next_node) {
                return Some(next_node);
            }
            next_node = self.get_next(next_node);
        }
    }
    pub fn remove_next_point(&mut self) -> Option<f64> {
        let node = loop {
            if self.remove_order_heap.is_empty() {
                return None;
            }
            let maybe_valid_node = self.remove_order_heap.pop().unwrap().0;
            if self.is_still_valid(&maybe_valid_node) {
                break maybe_valid_node;
            }
        };
        self.len -= 1;
        self.nodes[node.prev].next = node.next;
        self.nodes[node.next].prev = node.prev;
        self.update_value(node.prev);
        self.update_value(node.next);
        Some(node.value)
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    pub fn reduce_till_empty(&mut self) {
        while !self.is_empty() {
            if self.remove_next_point().is_none() {
                panic!("Ther are no more Points to remove");
            }
        }
    }
    pub fn get_triangles(&self) -> Triangulation {
        assert!(
            self.is_empty(),
            "Can't get triangles from non-empty reducer"
        );
        let mut triangles = Vec::with_capacity(self.nodes.len());
        for i in 0..self.nodes.len() {
            let node = &self.nodes[i];
            let prev = self.get_prev(node);
            let next = self.get_next(node);
            triangles.push(Triangle::new(prev.pos, node.pos, next.pos));
        }
        Triangulation::new(triangles)
    }
    pub fn get_polygon(&self) -> Polygon {
        let mut points = Vec::with_capacity(self.len);
        let mut node = self
            .get_next_valid_node(self.nodes.first().unwrap())
            .unwrap();
        for _ in 0..self.len {
            points.push(node.pos);
            node = self.get_next(node);
        }
        Polygon::new(points)
    }
}

pub trait RemoveOrder {
    fn get_value(prev: Vec2, this: Vec2, next: Vec2) -> Option<f64>;
}
