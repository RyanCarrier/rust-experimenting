#![feature(test)]
extern crate test;
extern crate rand;


use std::fmt;
#[derive(Debug, Clone, PartialEq)]
pub struct Arc {
    pub to: u64,
    pub distance: i64,
}
#[derive(Clone, PartialEq)]
pub struct Vertex {
    pub id: u64,
    pub best_distance: i64,
    pub best_verticie: u64,
    pub arcs: Vec<Arc>,
}

impl fmt::Debug for Vertex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\n\tid:{}\n\tbest_distance:{}\n\tbest_verticie:{}\n\tarcs:{:?}\n",
            self.id,self.best_distance,self.best_verticie,self.arcs,
        )
    }
}

impl Arc {
    pub fn str(&self) -> String {
        return format!("{},{}", self.to, self.distance);
    }
}


impl Vertex {
    pub fn new(id: u64) -> Vertex {
        Vertex {
            id: id,
            best_distance: 0,
            best_verticie: 0,
            arcs: Vec::new(),
        }
    }

    pub fn add_arc(&mut self, to: u64, distance: i64) {
        self.arcs.push(Arc {
            to: to,
            distance: distance,
        });
    }

    pub fn remove_arc(&mut self, to: u64) {
        self.arcs.retain(|a| a.to != to);
    }

    pub fn str(&self) -> String {
        return format!(
            "{} {}",
            self.id,
            (&self.arcs)
                .into_iter()
                .map(|a| a.str())
                .collect::<Vec<String>>()
                .join(" ")
        );
    }
}



#[derive(PartialEq, Clone)]
pub struct Graph {
    verticies: Vec<Vertex>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph { verticies: Vec::new() }
    }

    pub fn add_vertex(&mut self, v: Vertex) {
        self.verticies.push(v);
    }

    pub fn remove_vertex(&mut self, id: u64) {
        self.verticies.retain(|v| v.id != id)
    }
    pub fn str(&self) -> String {
        return (&self.verticies)
            .into_iter()
            .map(|v| v.str())
            .collect::<Vec<String>>()
            .join("\r\n");
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

    pub fn export4(&mut self) -> String {
        let mut i = 0;
        let l = self.verticies.len();
        let mut final_str: Vec<Vec<String>> = Vec::with_capacity(l as usize);

        while i < l {
            final_str.push(Vec::with_capacity(
                self.verticies[i].arcs.len() + 2 as usize,
            ));
            final_str[i].push(format!("{}", self.verticies[i].id));
            let al = self.verticies[i].arcs.len();
            let mut ai = 0;
            while ai < al {
                final_str[i].push(format!(
                    "{},{}",
                    self.verticies[i].arcs[ai].to,
                    self.verticies[i].arcs[ai].distance
                ));
                ai = ai + 1;
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


    pub fn export5(&self) -> String {
        let mut i = 0;
        let l = self.verticies.len();
        let mut final_str: Vec<Vec<String>> = Vec::with_capacity(l as usize);
        for v in &self.verticies {
            final_str.push(Vec::with_capacity(v.arcs.len() + 2 as usize));
            final_str[i].push(format!("{}", v.id));
            for a in &v.arcs {
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

    pub fn export6(&self) -> String {
        return self.str();
    }

    pub fn export7(&self) -> String {
        let mut final_str: String = String::new();
        for v in &self.verticies {
            let mut s = format!("{}", v.id);
            for a in &v.arcs {
                s.push_str(&format!(" {},{}", a.to, a.distance));
            }
            final_str.push_str(&s);
            final_str.push_str("\r\n");
        }
        return final_str;
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
    static LARGE_NODES: i64 = 1024;


    #[bench]
    fn a_small_export_1(b: &mut Bencher) {
        bench_n_i(0, 1, b);
    }
    #[bench]
    fn b_med_export_1(b: &mut Bencher) {
        bench_n_i(1, 1, b);
    }
    #[bench]
    fn c_large_export_1(b: &mut Bencher) {
        bench_n_i(2, 1, b);
    }



    #[bench]
    fn a_small_export_2(b: &mut Bencher) {
        bench_n_i(0, 2, b);
    }
    #[bench]
    fn b_med_export_2(_b: &mut Bencher) {
        print!("Too slow to export");
        //bench_n_i(1, 2, b);
    }
    #[bench]
    fn c_large_export_2(_b: &mut Bencher) {
        print!("Too slow to export");
        //bench_n_i(2, 2, b);
    }

    #[bench]
    fn a_small_export_3(b: &mut Bencher) {
        bench_n_i(0, 3, b);
    }
    #[bench]
    fn b_med_export_3(b: &mut Bencher) {
        bench_n_i(1, 3, b);
    }
    #[bench]
    fn c_large_export_3(b: &mut Bencher) {
        bench_n_i(2, 3, b);
    }

    #[bench]
    fn a_small_export_4(b: &mut Bencher) {
        bench_n_i(0, 4, b);
    }
    #[bench]
    fn b_med_export_4(b: &mut Bencher) {
        bench_n_i(1, 4, b);
    }
    #[bench]
    fn c_large_export_4(b: &mut Bencher) {
        bench_n_i(2, 4, b);
    }

    #[bench]
    fn a_small_export_5(b: &mut Bencher) {
        bench_n_i(0, 5, b);
    }
    #[bench]
    fn b_med_export_5(b: &mut Bencher) {
        bench_n_i(1, 5, b);
    }
    #[bench]
    fn c_large_export_5(b: &mut Bencher) {
        bench_n_i(2, 5, b);
    }

    #[bench]
    fn a_small_export_6(b: &mut Bencher) {
        bench_n_i(0, 6, b);
    }
    #[bench]
    fn b_med_export_6(b: &mut Bencher) {
        bench_n_i(1, 6, b);
    }
    #[bench]
    fn c_large_export_6(b: &mut Bencher) {
        bench_n_i(2, 6, b);
    }

    #[bench]
    fn a_small_export_7(b: &mut Bencher) {
        bench_n_i(0, 7, b);
    }
    #[bench]
    fn b_med_export_7(b: &mut Bencher) {
        bench_n_i(1, 7, b);
    }
    #[bench]
    fn c_large_export_7(b: &mut Bencher) {
        bench_n_i(2, 7, b);
    }



    fn bench_n_i(size: i64, export: i64, b: &mut Bencher) {
        //setup();
        let mut g: Graph;
        match size {
            0 => g = generate(SMALL_NODES),
            1 => g = generate(MEDIUM_NODES),
            2 => g = generate(LARGE_NODES),
            _ => panic!("err"),
        }
        match export {
            1 => b.iter(|| g.export1()),
            2 => b.iter(|| g.export2()),
            3 => b.iter(|| g.export3()),
            4 => b.iter(|| g.export4()),
            5 => b.iter(|| g.export5()),
            6 => b.iter(|| g.export6()),
            7 => b.iter(|| g.export7()),
            _ => panic!("wtf!"),
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
