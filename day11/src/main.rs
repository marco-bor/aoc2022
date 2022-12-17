use std::{cell::RefCell, str::FromStr};

use evalexpr::{
    build_operator_tree, Context, ContextWithMutableVariables, HashMapContext, Node, Value,
};

fn main() {
    let input = include_str!("input.txt");

    println!("Problem 1: {}", problem1(input));
    println!("Problem 2: {}", problem2(input));
}

fn problem1(input: &str) -> usize {
    let mut monkeys = parse_monkeys(input)
        .into_iter()
        .map(RefCell::new)
        .collect::<Vec<_>>();
    play_rounds(&monkeys, 20);
    monkeys.sort_by_key(|m| m.borrow().rounds);

    monkeys
        .iter()
        .rev()
        .take(2)
        .map(|m| m.borrow().rounds)
        .product()
}

fn play_rounds(monkeys: &[RefCell<Monkey>], rounds: usize) {
    for _ in 0..rounds {
        for monkey_ in monkeys {
            let mut monkey = monkey_.borrow_mut();
            let items = monkey.items.borrow().clone();
            monkey.rounds += items.len();
            monkey.items.borrow_mut().clear();

            for item in items {
                let new = monkey.eval(item) / 3;

                if new % monkey.test.div_by == 0 {
                    monkeys[monkey.test.if_true as usize]
                        .borrow()
                        .items
                        .borrow_mut()
                        .push(new);
                } else {
                    monkeys[monkey.test.if_false as usize]
                        .borrow()
                        .items
                        .borrow_mut()
                        .push(new);
                }
            }
        }
    }
}

fn play_rounds2(monkeys: &[RefCell<Monkey>], rounds: usize) {
    for _ in 0..rounds {
        for monkey_ in monkeys {
            let mut monkey = monkey_.borrow_mut();
            let items = monkey.items.borrow().clone();
            monkey.rounds += items.len();
            monkey.items.borrow_mut().clear();

            for item in items {
                let new = monkey.eval(item);

                if new % monkey.test.div_by == 0 {
                    monkeys[monkey.test.if_true as usize]
                        .borrow()
                        .items
                        .borrow_mut()
                        .push(new);
                } else {
                    monkeys[monkey.test.if_false as usize]
                        .borrow()
                        .items
                        .borrow_mut()
                        .push(new);
                }
            }
        }
    }
}

fn problem2(input: &str) -> usize {
    let mut monkeys = parse_monkeys(input)
        .into_iter()
        .map(RefCell::new)
        .collect::<Vec<_>>();
    play_rounds2(&monkeys, 1000);
    monkeys.sort_by_key(|m| m.borrow().rounds);

    monkeys
        .iter()
        .rev()
        .take(2)
        .map(|m| m.borrow().rounds)
        .product()
}

#[test]
fn test_problem1() {
    assert_eq!(problem1(include_str!("testdata.txt")), 10605);
}

#[test]
fn test_problem2() {
    assert_eq!(problem2(include_str!("testdata.txt")), 5204 * 5192);
}

struct Monkey {
    id: u8,
    items: RefCell<Vec<usize>>,
    operation: Node,
    test: Test,
    rounds: usize,
}

impl Monkey {
    fn eval(&self, old: usize) -> usize {
        let mut ctx = HashMapContext::new();
        ctx.set_value("old".to_owned(), Value::from(old as i64))
            .unwrap();
        self.operation.eval_with_context_mut(&mut ctx).unwrap();
        match ctx.get_value("new").unwrap() {
            Value::Int(value) => *value as usize,
            _ => unreachable!("wtf"),
        }
    }

    fn eval2(&self, old: usize) -> usize {
        let mut ctx = HashMapContext::new();
        ctx.set_value("old".to_owned(), Value::from(old as i64))
            .unwrap();
        self.operation.eval_with_context_mut(&mut ctx).unwrap();
        match ctx.get_value("new").unwrap() {
            Value::Int(value) => primes::factors(*value as u64).iter().product::<u64>() as usize,
            _ => unreachable!("wtf"),
        }
    }
}

struct Test {
    div_by: usize,
    if_true: u8,
    if_false: u8,
}

impl FromStr for Monkey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let id = lines
            .next()
            .unwrap()
            .split(&[':', ' ']).nth(1)
            .unwrap()
            .parse::<u8>()
            .unwrap();
        let items = lines
            .next()
            .unwrap()
            .split(&[',', ' ', ':'])
            .flat_map(|s| s.parse::<usize>())
            .collect::<Vec<_>>();

        let operation =
            build_operator_tree(lines.next().unwrap().split(':').nth(1).unwrap()).unwrap();

        let test = parse_test(lines);

        Ok(Self {
            id,
            items: RefCell::new(items),
            operation,
            test,
            rounds: 0,
        })
    }
}

fn parse_test<'a>(mut lines: impl Iterator<Item = &'a str>) -> Test {
    let div_by = lines
        .next()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let if_true = lines
        .next()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<u8>()
        .unwrap();
    let if_false = lines
        .next()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<u8>()
        .unwrap();
    Test {
        div_by,
        if_true,
        if_false,
    }
}

fn parse_monkeys(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .flat_map(|m| m.parse::<Monkey>())
        .collect::<Vec<_>>()
}
