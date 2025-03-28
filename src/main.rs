#![allow(
    unused_imports,
    non_snake_case,
    unused_variables,
    unused_mut,
    dead_code
)]
use std::collections::HashMap;

use clap::{arg, command, value_parser, Arg, ArgAction, Command};
use colored::{ColoredString, Colorize};

fn is_prime(&num: &u64) -> bool {
    let sqrt = num.isqrt();
    (2..=sqrt).all(|c| num % c != 0)
}

fn get_pq(num: u64) -> (u64, u64) {
    if is_prime(&num) {
        return (num, 1);
    }
    let sqrt = num.isqrt();
    for p in (3..=sqrt).filter(is_prime) {
        let q = num / p;
        if q * p == num {
            return (p, q);
        }
    }
    // blasphemmouse but we won't work with it anyway
    // its not a semiprime
    return (num, 1);
}

fn is_full_sqr(s: ColoredString, num: u64) -> ColoredString {
    if num == 0 {
        return s;
    }
    let sqrt = num.isqrt();
    let mut s: ColoredString = s;
    if sqrt * sqrt == num {
        s = s.yellow();
        if s.bgcolor.is_none() {
            s = s.black().on_yellow();
        }
    }
    s
}

fn is_pq_div(s: ColoredString, n: u64, p: u64, q: u64) -> ColoredString {
    let mut s = s;

    if p > 1 && n % p == 0 {
        s = s.red().on_cyan();
    }
    if q > 1 && n % q == 0 {
        if s.is_plain() {
            s = s.black();
        }
        s = s.on_bright_cyan();
    }
    return s;
}
fn get_seq_mark(valid: bool) -> ColoredString {
    if valid {
        "*".red()
    } else {
        " ".into()
    }
}
fn get_adj_mark(valid: bool) -> ColoredString {
    if valid {
        "#".red()
    } else {
        " ".into()
    }
}
fn main() {
    let matches = command!()
        .arg(
            arg!([N] "some semiprime")
                .required(true)
                .value_parser(value_parser!(u64)),
        )
        .arg(
            arg!(-m --maxlines <VALUE> "maximum lines in the output.\nif range is provided the count starts from lower bound")
            .value_parser(value_parser!(u64)),
        )
        .arg(
            Arg::new("BOUND")
                .short('r')
                .long("range")
                .num_args(2)
                .help("line range to print [low;high]")
                .value_parser(value_parser!(u64)),
        )
        .get_matches();
    if let Some(&N) = matches.get_one::<u64>("N") {
        let (p, q) = get_pq(N);
        if p == N || !is_prime(&q) {
            eprintln!("is not odd semiprime");
            return;
        }
        let mut cache: Vec<Option<bool>> = vec![None; N.isqrt() as usize];
        let mut lines = N / 2 + (N & 1);
        let mut maxlines = lines;
        if let Some(&mln) = matches.get_one::<u64>("maxlines") {
            maxlines = mln.min(lines);
        }
        let mut lo = 1;
        let mut hi = lines;
        if let Some(vec) = matches.get_many::<u64>("BOUND") {
            let mut vec = vec.collect::<Vec<_>>();
            let [low, high, ..] = vec[..] else {
                panic!("Insufficient args")
            };

            if lo >= hi {
                eprintln!("lower bound must be less than higher bound");
                return;
            }
            lo = lo.max(*low);
            hi = hi.min(*high + 1).min(*low + maxlines);
        }
        hi = hi.min(lo + maxlines).min(lines);

        let ws = N.to_string().len();
        let wb = (N * N).to_string().len();
        let mmod = wb.max(6);

        println!("N: {N} = a + b = p * q = {p} * {q} ");
        println!("M = a * b");
        println!("rl = b * b mod N \t rr = M mod N");
        println!("t = b - a \t t0 = (t - 1) / 2 \t t1 = t0 + 1\t tn = N - 1 - t");
        println!(
            "Colors: {}{}{}{}{}{}",
            is_full_sqr(" x^2 ".into(), 4),
            is_pq_div(" p divisible ".into(), p, p, q),
            is_pq_div(" q divisible ".into(), q, p, q),
            is_pq_div(" pq divisible ".into(), p * q, p, q),
            is_full_sqr(is_pq_div(" p*p ".into(), p * p, p, q), p * p),
            is_full_sqr(is_pq_div(" q*q ".into(), q * q, p, q), q * q),
        );
        fn get_seq(p: u64, n: u64) -> String {
            (0..p)
                .map(|k| n + 2 * k)
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join("+")
        }
        let n = if p < q { q - p + 1 } else { 1 };
        let mut k = 0;
        println!("{} - part of {N}={}", get_seq_mark(true), get_seq(p, n));
        println!("{} - x*(x+1)", get_adj_mark(true));
        println!();
        {
            // header section
            // "{rl_str}|{b_str}|{a_str}|{t_str}|{pp_str}|#{pp_mod_str}|*{tn_str}|{M_str}|{rr_str}"
            println!(
                " {:^ws$} | {:^ws$} | {:^ws$} | {:^ws$} | {:^wb$} |{} {:^mmod$} | {}{:^ws$} | {:^wb$} | {:^ws$}",
                "rl", "b", "a", "t", "pp"," ", "pp % N", " ","tn", "M", "rr"
            );
        }

        for i in lo..hi {
            let a = i;
            let a_fmt = format!(" {a:ws$} ");
            let a_str = is_pq_div(a_fmt.into(), a, p, q);

            let b = N - i;
            let b_fmt = format!(" {b:ws$} ");
            let b_str = is_pq_div(b_fmt.into(), b, p, q);

            let M = a * b;
            let M_fmt = std::format!(" {M:wb$} ");
            let mut M_str = is_full_sqr(is_pq_div(M_fmt.into(), M, p, q), M);

            let rl = (b * b) % N;
            let rl_fmt = std::format!(" {rl:ws$} ");
            let mut rl_str = is_full_sqr(is_pq_div(rl_fmt.into(), rl, p, q), rl);

            let rr = M % N;
            let rr_fmt = std::format!(" {rr:ws$} ");
            let mut rr_str = is_full_sqr(is_pq_div(rr_fmt.into(), rr, p, q), rr);

            let t = b - a;
            let t_fmt = std::format!(" {t:ws$} ");
            let mut t_str = is_full_sqr(is_pq_div(t_fmt.into(), t, p, q), t);

            let t0 = (t - 1) / 2;
            let t1 = t0 + 1;

            let tn = N - 1 - t;
            let in_seq = n + 2 * k == tn && k < p;
            if in_seq {
                k += 1;
            };
            let tn_fmt = std::format!(" {tn:ws$} ");
            let mut tn_str = is_full_sqr(is_pq_div(tn_fmt.into(), tn, p, q), tn);
            let mut tnseq = get_seq_mark(in_seq);
            if let Some(color) = tn_str.bgcolor {
                tnseq = tnseq.on_color(color);
            }

            let pp = t0 * t1;
            let pp_fmt = std::format!(" {pp:wb$} ");
            let mut pp_str = is_full_sqr(is_pq_div(pp_fmt.into(), pp, p, q), pp);

            let pp_mod = pp % N;
            let pp_sqrt = pp_mod.isqrt();
            let pp_mod_fmt = std::format!(" {pp_mod:mmod$} ");

            let mut pp_mod_str = is_full_sqr(is_pq_div(pp_mod_fmt.into(), pp_mod, p, q), pp_mod);
            let mut pp_adj_mark = get_adj_mark(pp_sqrt * (pp_sqrt + 1) == pp_mod);
            if let Some(color) = pp_mod_str.bgcolor {
                pp_adj_mark = pp_adj_mark.on_color(color);
            }
            println!(
                "{rl_str}|{b_str}|{a_str}|{t_str}|{pp_str}|{pp_adj_mark}{pp_mod_str}|{tnseq}{tn_str}|{M_str}|{rr_str}"
            );
        }
    }
}
