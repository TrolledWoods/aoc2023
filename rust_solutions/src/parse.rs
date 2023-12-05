pub fn parse<'a, T: Parse>(input: &'a str, mut parse: T) -> Option<T::Output<'a>> {
    let mut parser = Parser {
        input: input.trim(),
    };
    if parser.input.is_empty() {
        return None;
    }
    let v = parse.parse(&mut parser).expect("Parsing failed!");
    assert!(input.is_empty(), "Did not finish parsing!");
    Some(v)
}

pub trait Parse {
    type Output<'a> where Self: 'a;

    fn parse<'a>(&mut self, parser: &mut Parser<'a>) -> Result<Self::Output<'a>, ()>;
}

#[derive(Clone, Copy)]
pub struct Parser<'a> {
    input: &'a str,
}

pub struct List<Elem>(pub Elem);
impl<Elem: Parse> Parse for List<Elem> {
    type Output<'a> = Vec<Elem::Output<'a>> where Self: 'a;

    fn parse<'a>(&mut self, parser: &mut Parser<'a>) -> Result<Self::Output<'a>, ()> {
        let mut output = Vec::new();
        while let Ok(elem) = self.0.parse(parser) {
            output.push(elem);
        }
        Ok(output)
    }
}

impl Parse for &'static str {
    type Output<'a> = ();

    fn parse<'a>(&mut self, parser: &mut Parser<'a>) -> Result<Self::Output<'a>, ()> {
        if let Some(remainder) = parser.input.strip_prefix(*self) {
            parser.input = remainder.trim_start();
            Ok(())
        } else {
            Err(())
        }
    }
}

pub struct Int;

impl Parse for Int {
    type Output<'a> = i64;

    fn parse<'a>(&mut self, parser: &mut Parser<'a>) -> Result<i64, ()> {
        let mut c = parser.input.char_indices();
        if matches!(c.clone().next(), Some((_, '-'))) {
            c.next();
        }

        while c.clone().next().is_some_and(|v| v.1.is_ascii_digit()) {
            c.next();
        }

        let num = parser.input[..c.offset()].parse().map_err(|_v| ())?;
        parser.input = c.as_str();

        Ok(num)
    }
}

macro_rules! impl_parse_for_tuple {
    () => {};
    ($i1: ident: $t1: ident, $($i:ident: $t:ident,)*) => {
        impl<$t1: Parse, $($t: Parse,)*> Parse for (($t1, $($t,)*)) {
            type Output<'a> = ($t1::Output<'a>, $($t::Output<'a>,)*) where Self: 'a;

            fn parse<'a>(&mut self, parser: &mut Parser<'a>) -> Result<Self::Output<'a>, ()> {
                let original = *parser;
                let ($i1, $($i,)*) = self;

                let $i1 = match $i1.parse(parser) {
                    Ok(v) => v,
                    Err(()) => {
                        *parser = original;
                        return Err(());
                    }
                };

                $(
                    let $i = match $i.parse(parser) {
                        Ok(v) => v,
                        Err(()) => {
                            *parser = original;
                            return Err(());
                        }
                    };
                )*

                Ok(($i1, $($i,)*))
            }
        }

        impl_parse_for_tuple!($($i: $t,)*);
    };
}

impl_parse_for_tuple!(a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H,);