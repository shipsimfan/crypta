macro_rules! round {
    ($f: ident, $a: ident, $b: ident, $c: ident, $d: ident, $k: literal, $s: literal, $i: literal, $x: ident) => {
        ff(&mut $a, $b, $c, $d, $x[$k], $s, T[$i], $f);
    };
}
pub(super) use round;
