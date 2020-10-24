#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(
        n: usize,
        m: usize,
        a: [isize; n],
        b: [isize; n],
        cd: [(Usize1, Usize1); m]
    );

    let mut dsu = dsu::Dsu::new(n);

    for (r, l) in cd {
        dsu.unite(r, l);
    }

    let aa = dsu.get_all_groups();
    for (_root, set) in aa {
        let mut a_sum: isize = 0;
        let mut b_sum: isize = 0;

        for j in set {
            a_sum += a[j];
            b_sum += b[j];
        }
        if a_sum != b_sum {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
#[allow(dead_code)]
mod dsu {
    #[derive(Debug, Clone)]
    enum Node {
        Root(usize),
        Child(usize),
    }
    ///UnionFind
    #[derive(Clone, Debug)]
    pub struct Dsu {
        uf: Vec<Node>,
    }

    impl Dsu {
        pub fn new(n: usize) -> Dsu {
            Dsu {
                uf: vec![Node::Root(1); n],
            }
        }

        pub fn root(&mut self, target: usize) -> usize {
            match self.uf[target] {
                Node::Root(_) => target,
                Node::Child(par) => {
                    let root = self.root(par);
                    self.uf[target] = Node::Child(self.root(par));
                    root
                }
            }
        }
        pub fn unite(&mut self, x: usize, y: usize) -> bool {
            let rx = self.root(x);
            let ry = self.root(y);
            if rx == ry {
                return false;
            }
            let size_x = self.size(x);
            let size_y = self.size(y);

            let (i, j) = if size_x > size_y { (rx, ry) } else { (ry, rx) };
            self.uf[i] = Node::Root(size_x + size_y);
            self.uf[j] = Node::Child(i);

            true
        }
        pub fn is_same(&mut self, x: usize, y: usize) -> bool {
            self.root(x) == self.root(y)
        }
        pub fn get_same_group(&mut self, x: usize) -> std::collections::HashSet<usize> {
            let root = self.root(x);
            let mut g = std::collections::HashSet::new();
            for i in 0..self.uf.len() {
                if root == self.root(i) {
                    g.insert(i);
                }
            }
            g
        }
        pub fn get_all_groups(
            &mut self,
        ) -> std::collections::HashMap<usize, std::collections::HashSet<usize>> {
            let mut map: std::collections::HashMap<usize, std::collections::HashSet<usize>> =
                std::collections::HashMap::new();
            for i in 0..self.uf.len() {
                let root = self.root(i);

                map.entry(root)
                    .or_insert_with(std::collections::HashSet::new)
                    .insert(i);
            }
            map
        }

        pub fn size(&mut self, x: usize) -> usize {
            let root = self.root(x);
            match self.uf[root] {
                Node::Root(size) => size,
                Node::Child(_) => 0,
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn dsu_works() {
            let mut d = Dsu::new(4);
            d.unite(0, 1);
            assert_eq!(d.is_same(0, 1), true);
            d.unite(1, 2);
            assert_eq!(d.is_same(0, 2), true);
            assert_eq!(d.size(0), 3);
            assert_eq!(d.is_same(0, 3), false);
            // assert_eq!(d.groups(), vec![vec![0, 1, 2], vec![3]]);
        }
    }
}
