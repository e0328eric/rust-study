#![allow(unused)]
type Vertex = isize;
type Graph = Vec<(isize, isize)>;

fn is_in_graph(g: &Graph, v: &Vertex) -> bool {
    let mut lst:Vec<bool> = Vec::new();
    for f in g {
        lst.push(*v == f.0 || *v == f.1);
    }
    lst.iter().any(|&x| x)
}

fn search_to_go(g: &Graph, v: &Vertex) -> Vec<Vertex> {
    g.iter().filter(|x| *v == x.0).map(|x| x.1).collect()
}

fn search_to_path(g: &Graph, mut pass_lst: &mut Vec<Vertex>, pres_lst: &Vec<Vertex>) -> Vec<Vertex> {
    let empty: Vec<Vertex> = Vec::new();
    if pres_lst.is_empty() {
        return empty;
    } else {
        let pres = pres_lst[0];
        let xs = Vec::from(&pres_lst[1..]);
        if !is_in_graph(&g, &pres) {
        return empty;
        } else if search_to_go(&g, &pres).is_empty() {
            pass_lst.push(pres);
            let mut lst = search_to_path(&g, &mut pass_lst, &xs);
            lst.push(pres);
            return lst;
        } else if pass_lst.contains(&pres) {
            return search_to_path(&g, &mut pass_lst, &xs);
        } else {
            pass_lst.push(pres);
            let updated_to_go = vec![search_to_go(&g, &pres), xs].concat();
            let mut lst = search_to_path(&g, &mut pass_lst, &updated_to_go);
            lst.push(pres);
            return lst;
        }
    }
}

fn sort_vec(lst: &Vec<isize>) -> Vec<isize> {
    if lst.is_empty() {
        Vec::new()
    } else {
        let mut output:Vec<isize> = Vec::new();
        let sorted_xs = sort_vec(&Vec::from(&lst[1..]));
        for i in sorted_xs.iter().filter(|t| **t < lst[0]) {
            output.push(*i);
        }
        output.push(lst[0]);
        for i in sorted_xs.iter().filter(|t| **t > lst[0]) {
            output.push(*i);
        }
        output
    }
}

fn reach(g: &Graph, v: Vertex) -> Vec<Vertex> {
    let mut tmp = search_to_path(&g, &mut Vec::new(), &vec![v]);
    sort_vec(&tmp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_graph() {
        let graph = vec![(1,2),(2,3)];
        assert_eq!(reach(&graph, 1), vec![1,2,3]);
        assert_eq!(reach(&graph, 2), vec![2,3]);
        assert_eq!(reach(&graph, 3), vec![3]);
    }

    #[test]
    fn loop_graph() {
        let graph = vec![(1,2),(2,3), (3,4), (4,1)];
        assert_eq!(reach(&graph, 1), vec![1,2,3,4]);
        assert_eq!(reach(&graph, 2), vec![1,2,3,4]);
        assert_eq!(reach(&graph, 3), vec![1,2,3,4]);
        assert_eq!(reach(&graph, 4), vec![1,2,3,4]);
    }

    #[test]
    fn complex_graph() {
        let graph = vec![(1,2),(2,3),(3,4),(4,2),(2,5)];
        assert_eq!(reach(&graph, 1), vec![1,2,3,4,5]);
        assert_eq!(reach(&graph, 2), vec![2,3,4,5]);
        assert_eq!(reach(&graph, 3), vec![2,3,4,5]);
        assert_eq!(reach(&graph, 4), vec![2,3,4,5]);
        assert_eq!(reach(&graph, 5), vec![5]);
    }

    #[test]
    fn very_complex_graph() {
        let graph = vec![(1,2),(1,5),(2,3),(3,4),(3,6),(5,6),(6,7),(7,2)];
        assert_eq!(reach(&graph, 1), vec![1,2,3,4,5,6,7]);
        assert_eq!(reach(&graph, 2), vec![2,3,4,6,7]);
        assert_eq!(reach(&graph, 3), vec![2,3,4,6,7]);
        assert_eq!(reach(&graph, 4), vec![4]);
        assert_eq!(reach(&graph, 5), vec![2,3,4,5,6,7]);
        assert_eq!(reach(&graph, 6), vec![2,3,4,6,7]);
        assert_eq!(reach(&graph, 7), vec![2,3,4,6,7]);
    }

    #[test]
    fn very_very_complex_graph() {
        let graph = vec![(1,2),(2,3),(3,4),(4,5),(5,1),(3,6),(6,7),(4,7),(5,8)];
        assert_eq!(reach(&graph, 1), vec![1,2,3,4,5,6,7,8]);
        assert_eq!(reach(&graph, 2), vec![1,2,3,4,5,6,7,8]);
        assert_eq!(reach(&graph, 3), vec![1,2,3,4,5,6,7,8]);
        assert_eq!(reach(&graph, 4), vec![1,2,3,4,5,6,7,8]);
        assert_eq!(reach(&graph, 5), vec![1,2,3,4,5,6,7,8]);
        assert_eq!(reach(&graph, 6), vec![6,7]);
        assert_eq!(reach(&graph, 7), vec![7]);
        assert_eq!(reach(&graph, 8), vec![8]);
    }
}

fn main() {
    println!("Hello, world!");
}
