// xfail-win32
use std;
import task;

fn f() {
    task::unsupervise();
    let a = ~0;
    fail;
}

fn main() {
    task::spawn {|| f(); };
}