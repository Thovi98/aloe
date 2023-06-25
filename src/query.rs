use crate::term::Term;
use crate::atom::Atom;
use crate::predicate::Predicate;
use crate::substitution::Substitution;
use crate::knowledge::Knowledge;
use std::collections::VecDeque;

// trait SearchStrategy {
//     type Item
//     fn add_states
// }

#[derive(Clone, Debug)]
struct State {
    query: VecDeque<Atom>,
    substitution: Substitution,
    depth: u32,
}

impl State {
    pub fn from(atom: Atom) -> Self {
        State {
            query: VecDeque::from([atom]),
            substitution: Substitution::new(),
            depth: 0,
        }
    }
}

// struct StateVec<'a> {
//     states: VecDeque<State<'a>>,
// }

// impl StateVec<'_> {
//     pub fn from(atom: &Atom) -> Self {
//         let state0 = State::from(atom);
//         StateVec {
//             states: VecDeque::from([state0])
//         }
//     }

//     pub fn expand(&mut self, template_query){
//     }
// }


// pub struct Goal {
//     goal: Vec<Atom>,
// }
// impl Goal {
//     pub fn play_with_states(&self) -> Option<State> {
//         match self.goal[..] {
//             [atom] => {
//                 let state = State::from(atom);
//                 Some(state)
//             },
//             _ => None,
//         }
//     }
//     // pub fn knowledge(&self, knowledge: &Knowledge, search_strategy: &SearchStrategy) {
//     // }
// }

// pub struct GoalBuilder {
//     goal: Vec<Atom>,
// }
// impl GoalBuilder {
//     pub fn new() -> Self {
//         GoalBuilder { goal: vec![] }
//     }
    
//     pub fn add_atom(mut self, atom: &Atom) -> Self {
//         self.goal.push(atom.clone());
//         self
//     }
    
//     pub fn build(self) -> Goal {
//         Goal { goal: self.goal }
//     }
// }



fn rename_variables(depth: u32) -> Box<dyn Fn(&Term) -> Term> {
    Box::new(move |term: &Term| {
        match term {
            Term::Variable(name) => {
                let new_name: String = format!("{}_{}", name.clone(), depth);
                Term::Variable(new_name)
            },
            _ => return term.clone(), 
        }
    })
}

pub fn query(fact: &Atom, knowledge: &Knowledge) -> Option<Vec<Substitution>> {
    let state0 = State {
        query: VecDeque::from([fact.clone()]),
        substitution: Substitution::new(),
        depth: 0,
    };
    let mut states = VecDeque::from([state0]);
    let mut solutions: Vec<Substitution> = vec![];

    while !states.is_empty() {
        let State { mut query, substitution, depth } = states.pop_front().unwrap();

        let fact = match query.pop_front() {
            None => {
                // When there are no fact to query, we found ourselves a solution
                solutions.push(substitution);
                println!("Found");
                continue
            },
            Some(fact) => fact,
        };

        for clause in knowledge.get_clauses() {
            // Change variable names of clause
            let clause = clause.apply_on_elements(&rename_variables(depth+1));

            // Unify head of clause and fact
            let returned_subst = match Substitution::unify_atom(&clause.head, &fact) {
                Err(_) => continue,
                Ok(returned_subst) => returned_subst,
            };
            println!("{:?}", returned_subst);

            // Merge all substitutions
            let mut new_subst = substitution.clone();
            if let Err(_) = new_subst.merge(&returned_subst) {
                continue
            };
            println!("{:?}", new_subst);

            // Apply substitution on query and on p.body
            let mut new_query: VecDeque<Atom> = query.iter().map(|x| new_subst.apply_on_atom(x)).collect();
            for atom in clause.body.iter().rev().map(|x| new_subst.apply_on_atom(x)){
                new_query.push_back(atom);
            }

            println!("{:?}", query);
            println!("{:?}", &new_query);
            // Add new state
            states.push_front(State { query: new_query, substitution: new_subst, depth: depth+1 });
        }
    }

    if solutions.len() > 0 { 
        Some(solutions)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matching_1() {
        let q = Atom {
            functor: String::from("mortal"),
            params: vec![ Term::StrCst("socrates".to_string()) ],
        };
        let pred = Predicate {
            head: Atom { functor: "mortal".to_string(), params: vec![ Term::Variable("X".to_string()) ] },
            body: vec![ Atom { functor: "human".to_string(), params: vec![ Term::Variable("X".to_string()) ] }],
        };
        let fact = Predicate {
            head: Atom { functor: "human".to_string(), params: vec![ Term::StrCst("socrates".to_string()) ] },
            body: vec![],
        };
        let knowledge = Knowledge { clauses: vec![pred, fact] };
        let output = query(&q, &knowledge);
        println!("{:?}", output);
        match &output {
            Some(sol) => println!("{:?}", sol),
            None => println!("?"),
        };
        assert!(Option::is_some(&output));
        // assert!(Atom::matching(&atom1, &atom1));
        // assert!(Atom::matching(&atom2, &atom2));
        // assert!(!Atom::matching(&atom1, &atom2));
    }
}
