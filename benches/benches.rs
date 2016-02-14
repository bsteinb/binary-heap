#![feature(test)]

extern crate binary_heap;
extern crate test;

use binary_heap::BinaryHeap;

use test::Bencher;

const N: u64 = 1024;
const N2: u64 = N * N;

fn make_heap() -> BinaryHeap<u64> {
    let xs: Vec<u64> = (0 .. N2).collect();
    BinaryHeap::from(xs)
}

#[bench]
fn pop_modify_push(b: &mut Bencher) {
    let mut heap = make_heap();
    b.iter(|| {
        for _ in 0 .. N {
            heap.pop().map(|elt| heap.push(elt + N2));
        }
    });
    test::black_box(heap.pop());
}

#[bench]
fn peek_mut(b: &mut Bencher) {
    let mut heap = make_heap();
    b.iter(|| {
        for _ in 0 .. N {
            heap.peek_mut(|elt| *elt = *elt + N2);
        }
    });
    test::black_box(heap.pop());
}

#[bench]
fn pop_push(b: &mut Bencher) {
    let mut heap = make_heap();
    b.iter(|| {
        for _ in 0 .. N {
            heap.pop_push(|elt| elt.map(|elt| elt + N2));
        }
    });
    test::black_box(heap.pop());
}

#[bench]
fn pop_and_then_push(b: &mut Bencher) {
    let mut heap = make_heap();
    b.iter(|| {
        for _ in 0 .. N {
            heap.pop_and_then_push(|elt| Some(elt + N2));
        }
    });
    test::black_box(heap.pop());
}

#[bench]
fn head_ref(b: &mut Bencher) {
    let mut heap = make_heap();
    b.iter(|| {
        for _ in 0 .. N {
            heap.head_ref().as_mut().map(|elt| *elt = *elt + N2);
        }
    });
    test::black_box(heap.pop());
}

#[bench]
fn head_take(b: &mut Bencher) {
    let mut heap = make_heap();
    b.iter(|| {
        for _ in 0 .. N {
            let mut head = heap.head();
            if let Some(elt) = head.take() {
                head.put(elt + N2);
            }
        }
    });
    test::black_box(heap.pop());
}

#[bench]
fn head_as_mut(b: &mut Bencher) {
    let mut heap = make_heap();
    b.iter(|| {
        for _ in 0 .. N {
            heap.head().as_mut().map(|elt| *elt = *elt + N2);
        }
    });
    test::black_box(heap.pop());
}
