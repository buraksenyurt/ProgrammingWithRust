#[allow(unused_macros)]
macro_rules! max_of {
    ($x:expr) => {
        $x
    };
    ($x:expr,$y:expr) => {
        if $x > $y {
            $x
        } else {
            $y
        }
    };
    ($x:expr,$($y:expr),+) => {
        max_of!($x,max_of!($($y),+))
    }
}

#[allow(unused_macros)]
macro_rules! crud {
    ($struct_name:ident, $($field_name:ident: $field_type:ty),*) => {
        #[derive(Debug)]
        struct $struct_name {
            $(
                $field_name: $field_type,
            )*
        }

        impl $struct_name {
            fn new($($field_name: $field_type),*) -> $struct_name {
                $struct_name { $($field_name),* }
            }
        }
    };
}

crud!(Product, id: i32,title: String,list_price:f32,category: String);

#[allow(unused_macros)]
macro_rules! wt {
    ($block:block) => {{
        let start = std::time::Instant::now();
        let result = $block;
        let duration = start.elapsed();
        println!("Total execution time: {:?}", duration);
        result
    }};
}

#[allow(unused_macros)]
macro_rules! xml {
    ($tag:ident { $($key:ident : $value:expr),* }) => {
        format!(
            "<{tag} {attributes} />",
            tag = stringify!($tag),
            attributes = vec![$(format!("{}=\"{}\"", stringify!($key), $value)),*].join(" ")
        )
    };
}

#[allow(unused_macros)]
macro_rules! wt_with {
    ($writer:expr, $block:block) => {{
        use std::io::Write;
        let start = std::time::Instant::now();
        let result = $block;
        let duration = start.elapsed();
        writeln!($writer, "Total execution time: {:?}", duration).unwrap();
        result
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xml_test() {
        let data = xml! {
            game {
                id:1,
                title:"Pacman 1983",
                rate:9.6
            }
        };
        assert_eq!(
            data,
            "<game id=\"1\" title=\"Pacman 1983\" rate=\"9.6\" />".to_string()
        );
    }
    #[test]
    fn wt_with_test() {
        let sum = wt_with!(std::io::stdout(), {
            let mut total = 0;
            for i in 1..100 {
                total += i;
            }
            total
        });
        assert_eq!(sum, 4950);
    }
    #[test]
    fn wt_test() {
        let sum = wt!({
            let mut total = 0;
            for i in 1..100 {
                total += i;
            }
            total
        });
        assert_eq!(sum, 4950);
    }

    #[test]
    fn test_crud_macro() {
        let c64 = Product::new(
            1,
            "C64 monitor 14.1inch".to_string(),
            999.99,
            "Retro Computer".to_string(),
        );
        assert_eq!(c64.id, 1);
        assert_eq!(c64.title, "C64 monitor 14.1inch".to_string());
        assert_eq!(c64.list_price, 999.99);
        assert_eq!(c64.category, "Retro Computer".to_string());
    }

    #[test]
    fn test_max_macro() {
        assert_eq!(max_of!(1), 1);
        assert_eq!(max_of!(2, 7), 7);
        assert_eq!(max_of!(10, 0, 6, 19, 20, 3, 2, 7), 20);
        assert_eq!(max_of!(-8, -5, -3, -99), -3);
    }
}
