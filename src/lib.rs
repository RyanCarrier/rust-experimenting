#![feature(test)]
extern crate test;
extern crate dijkstrust;
extern crate rand;

use dijkstrust::vertex;



#[derive(PartialEq, Clone)]
pub struct Graph {
    verticies: Vec<vertex::Vertex>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph { verticies: Vec::new() }
    }

    pub fn add_vertex(&mut self, v: vertex::Vertex) {
        self.verticies.push(v);
    }

    pub fn remove_vertex(&mut self, id: u64) {
        self.verticies.retain(|v| v.id != id)
    }
    pub fn export1_backup(&mut self) {
        (self.clone()).xport1();
    }

    pub fn xport1(self) -> String {
        let mut i = 0;
        let l = self.verticies.len();
        let mut final_str: Vec<Vec<String>> = Vec::with_capacity(l as usize);
        for v in self.verticies {
            final_str.push(Vec::with_capacity(v.arcs.len() + 2 as usize));
            final_str[i].push(format!("{}", v.id));
            for a in v.arcs {
                final_str[i].push(format!("{},{}", a.to, a.distance));
            }
            final_str[i].push(format!("\r\n"));
            i = i + 1;
        }
        let mut f: Vec<String> = Vec::with_capacity(l as usize);
        for x in final_str {
            f.push(x.join(" "))
        }
        return f.join("");
    }

    pub fn export1(&mut self) -> String {
        let mut i = 0;
        let l = self.verticies.len();
        let mut final_str: Vec<Vec<String>> = Vec::with_capacity(l as usize);
        for v in self.verticies.clone() {
            final_str.push(Vec::with_capacity(v.arcs.len() + 2 as usize));
            final_str[i].push(format!("{}", v.id));
            for a in v.arcs {
                final_str[i].push(format!("{},{}", a.to, a.distance));
            }
            final_str[i].push(format!("\r\n"));
            i = i + 1;
        }
        let mut f: Vec<String> = Vec::with_capacity(l as usize);
        for x in final_str {
            f.push(x.join(" "))
        }
        return f.join("");
    }

    pub fn export2(&mut self) -> String {
        let mut s = String::new();
        for v in self.verticies.clone() {
            s = format!("{}{}", s, v.id);
            for a in v.arcs {
                s = format!("{} {},{}", s, a.to, a.distance);
            }
            s = format!("{}\r\n", s);
        }
        return s;
    }

    pub fn export3(&mut self) -> String {
        let l = self.verticies.len();
        let mut final_str: Vec<String> = Vec::with_capacity(l as usize);
        for v in self.verticies.clone() {
            let mut s = format!("{}", v.id);
            for a in v.arcs {
                s = format!("{} {},{}", s, a.to, a.distance);
            }
            final_str.push(s);
        }
        return final_str.join("\r\n");
    }
}


#[cfg(test)]
mod bench {
    //extern crate test;
    use test::Bencher;
    use rand;
    use super::*;
    static SMALL_NODES: i64 = 16;
    static MEDIUM_NODES: i64 = 256;

    use dijkstrust::vertex::Vertex;

    #[bench]
    fn small_export_1(b: &mut Bencher) {
        bench_n_i(0, 1, b);
    }
    #[bench]
    fn med_export_1(b: &mut Bencher) {
        bench_n_i(1, 1, b);
    }

    #[bench]
    fn small_export_2(b: &mut Bencher) {
        bench_n_i(0, 2, b);
    }
    #[bench]
    fn med_export_2(b: &mut Bencher) {
        bench_n_i(1, 2, b);
    }

    #[bench]
    fn small_export_3(b: &mut Bencher) {
        bench_n_i(0, 3, b);
    }
    #[bench]
    fn med_export_3(b: &mut Bencher) {
        bench_n_i(1, 3, b);
    }

    fn bench_n_i(size: i64, export: i64, b: &mut Bencher) {
        //setup();
        let mut g: Graph;
        match size {
            0 => g = generate(SMALL_NODES),
            1 => g = generate(MEDIUM_NODES),
            _ => panic!("err"),
        }
        if export == 1 {
            b.iter(|| g.export1());
        } else if export == 2 {
            b.iter(|| g.export2());
        } else if export == 3 {
            b.iter(|| g.export3());
        } else {
            panic!("wtff");
        }
    }

    fn generate(nodes: i64) -> Graph {
        let mut g = Graph::new();
        for i in 0..nodes as i64 {
            let mut v = Vertex::new(i as u64);
            for j in 0..nodes as i64 {
                if j == i {
                    continue;
                }
                //u32 to i64, to ensure >0
                let r = rand::random::<u32>() as i64 % ((nodes * (nodes - j + 1)) as i64);
                v.add_arc(j as u64, ((2 * nodes) - j) as i64 + (r));
            }
            g.add_vertex(v);
        }
        return g;
    }

}
