// 参考 https://doc.rust-lang.org/reference/macros-by-example.html
macro_rules! read {
    () => { read!('next) };
    ($t:tt) => { // let ss = read!('line).split(',').collect::<Vec<_>>();
        read_inner! {
            std::io::stdin()
                .lock()
                .bytes()
                .map(|u| u.unwrap() as char)
                .skip_while(|&c| c.is_whitespace() /* 先頭の空白は無視 */), $t
        }
    };
    ($t:tt; $length:expr) => { // let v = read!(i32; 100);
        read!([$t; $length])
    };
    ($t:tt $(, $rest_t:tt)* $(,)?) => { // let (i, j) = read!(i32, i64);
        read!(($t $(, $rest_t)* ,))
    };
    ($($($i:ident)+ : $t:tt $(,)? $(;)?)*) => { // read! { let i: i32; j: i32; }
        $($($i)+ = read!($t);)*
    };
}

macro_rules! read_inner {
    ($iter:expr, ['seqc; $length:expr]) => { // 連続する $length 文字
        $iter.take($length).collect::<Vec<_>>()
    };
    ($iter:expr, [$t:tt; $length:expr]) => { // vec
        (0..$length).map(|_| read_inner!($iter, $t)).collect::<Vec<_>>()
    };
    ($iter:expr, ()) => { // void (読み飛ばし)
        (read_inner!($iter, 'next), ()).1
    };
    ($iter:expr, ($t:tt ,)) => { // tuple (要素 1)
        (read_inner!($iter, $t) ,)
    };
    ($iter:expr, ($t:tt $(, $rest_t:tt)+ $(,)?)) => { // tuple
        (read_inner!($iter, $t) $(, read_inner!($iter, $rest_t))+)
    };
    ($iter:expr, 'next) => { // 次 (空白区切り)
        $iter.take_while(|&c| !c.is_whitespace()).collect::<String>()
    };
    ($iter:expr, 'line) => { // 次 (改行区切り)
        $iter.take_while(|&c| c != '\n' && c != '\r').collect::<String>()
    };
    ($iter:expr, char) => {
        $iter.next().unwrap()
    };
    ($iter:expr, $t:tt) => { // 本質
        read_inner!($iter, 'next).parse::<$t>().unwrap()
    };
}
