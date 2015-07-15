#![allow(unused_imports)]
use util::tok::Tok;

#[allow(non_snake_case)]
pub fn parse_S<
    __TOKENS: IntoIterator<Item=Tok>,
>(
    __tokens: __TOKENS,
) -> Result<(Option<Tok>, i32), Option<Tok>>
{
    let mut __tokens = __tokens.into_iter();
    let __lookahead = __tokens.next();
    match try!(__parse__S::__state0(__lookahead, &mut __tokens)) {
        (__lookahead, __parse__S::__Nonterminal::____S(__nt)) => Ok((__lookahead, __nt)),
        _ => unreachable!(),
    }
}

mod __parse__S {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use util::tok::Tok;

    pub enum __Nonterminal<> {
        E(i32),
        T(i32),
        ____S(i32),
        S(i32),
    }

    // State 0
    //   E = (*) E "-" T [EOF]
    //   E = (*) E "-" T ["-"]
    //   E = (*) T [EOF]
    //   E = (*) T ["-"]
    //   S = (*) E [EOF]
    //   T = (*) "(" E ")" [EOF]
    //   T = (*) "(" E ")" ["-"]
    //   T = (*) "Num" [EOF]
    //   T = (*) "Num" ["-"]
    //   __S = (*) S [EOF]
    //
    //   "(" -> Shift(S5)
    //   "Num" -> Shift(S4)
    //
    //   E -> S1
    //   T -> S2
    //   S -> S3
    pub fn __state0<
        __TOKENS: Iterator<Item=Tok>,
    >(
        mut __lookahead: Option<Tok>,
        __tokens: &mut __TOKENS,
    ) -> Result<(Option<Tok>, __Nonterminal<>), Option<Tok>>
    {
        let mut __result: (Option<Tok>, __Nonterminal<>);
        match __lookahead {
            Some(__tok @ Tok::LParen(..)) => {
                let mut __sym0 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state5(__lookahead, __tokens, __sym0));
            }
            Some(Tok::Num(__tok0)) => {
                let mut __sym0 = &mut Some((__tok0));
                let __lookahead = __tokens.next();
                __result = try!(__state4(__lookahead, __tokens, __sym0));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        loop {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::E(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state1(__lookahead, __tokens, __sym0));
                }
                __Nonterminal::T(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state2(__lookahead, __tokens, __sym0));
                }
                __Nonterminal::S(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state3(__lookahead, __tokens, __sym0));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
    }

    // State 1
    //   E = E (*) "-" T [EOF]
    //   E = E (*) "-" T ["-"]
    //   S = E (*) [EOF]
    //
    //   EOF -> Reduce(S = E => ActionFn(1);)
    //   "-" -> Shift(S6)
    //
    pub fn __state1<
        __TOKENS: Iterator<Item=Tok>,
    >(
        mut __lookahead: Option<Tok>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<i32>,
    ) -> Result<(Option<Tok>, __Nonterminal<>), Option<Tok>>
    {
        let mut __result: (Option<Tok>, __Nonterminal<>);
        match __lookahead {
            Some(__tok @ Tok::Minus(..)) => {
                let mut __sym1 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state6(__lookahead, __tokens, __sym0, __sym1));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__actions::__action1(__sym0);
                return Ok((__lookahead, __Nonterminal::S(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        return Ok(__result);
    }

    // State 2
    //   E = T (*) [EOF]
    //   E = T (*) ["-"]
    //
    //   "-" -> Reduce(E = T => ActionFn(3);)
    //   EOF -> Reduce(E = T => ActionFn(3);)
    //
    pub fn __state2<
        __TOKENS: Iterator<Item=Tok>,
    >(
        mut __lookahead: Option<Tok>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<i32>,
    ) -> Result<(Option<Tok>, __Nonterminal<>), Option<Tok>>
    {
        let mut __result: (Option<Tok>, __Nonterminal<>);
        match __lookahead {
            Some(Tok::Minus(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__actions::__action3(__sym0);
                return Ok((__lookahead, __Nonterminal::E(__nt)));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__actions::__action3(__sym0);
                return Ok((__lookahead, __Nonterminal::E(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
    }

    // State 3
    //   __S = S (*) [EOF]
    //
    //   EOF -> Reduce(__S = S => ActionFn(0);)
    //
    pub fn __state3<
        __TOKENS: Iterator<Item=Tok>,
    >(
        mut __lookahead: Option<Tok>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<i32>,
    ) -> Result<(Option<Tok>, __Nonterminal<>), Option<Tok>>
    {
        let mut __result: (Option<Tok>, __Nonterminal<>);
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__actions::__action0(__sym0);
                return Ok((__lookahead, __Nonterminal::____S(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
    }

    // State 4
    //   T = "Num" (*) [EOF]
    //   T = "Num" (*) ["-"]
    //
    //   "-" -> Reduce(T = "Num" => ActionFn(4);)
    //   EOF -> Reduce(T = "Num" => ActionFn(4);)
    //
    pub fn __state4<
        __TOKENS: Iterator<Item=Tok>,
    >(
        mut __lookahead: Option<Tok>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<i32>,
    ) -> Result<(Option<Tok>, __Nonterminal<>), Option<Tok>>
    {
        let mut __result: (Option<Tok>, __Nonterminal<>);
        match __lookahead {
            Some(Tok::Minus(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__actions::__action4(__sym0);
                return Ok((__lookahead, __Nonterminal::T(__nt)));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__actions::__action4(__sym0);
                return Ok((__lookahead, __Nonterminal::T(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
    }

    // State 5
    //   E = (*) E "-" T [")"]
    //   E = (*) E "-" T ["-"]
    //   E = (*) T [")"]
    //   E = (*) T ["-"]
    //   T = (*) "(" E ")" [")"]
    //   T = (*) "(" E ")" ["-"]
    //   T = "(" (*) E ")" [EOF]
    //   T = "(" (*) E ")" ["-"]
    //   T = (*) "Num" [")"]
    //   T = (*) "Num" ["-"]
    //
    //   "(" -> Shift(S8)
    //   "Num" -> Shift(S10)
    //
    //   T -> S7
    //   E -> S9
    pub fn __state5<
        __TOKENS: Iterator<Item=Tok>,
    >(
        mut __lookahead: Option<Tok>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
    ) -> Result<(Option<Tok>, __Nonterminal<>), Option<Tok>>
    {
        let mut __result: (Option<Tok>, __Nonterminal<>);
        match __lookahead {
            Some(__tok @ Tok::LParen(..)) => {
                let mut __sym1 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state8(__lookahead, __tokens, __sym1));
            }
            Some(Tok::Num(__tok0)) => {
                let mut __sym1 = &mut Some((__tok0));
                let __lookahead = __tokens.next();
                __result = try!(__state10(__lookahead, __tokens, __sym1));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        while __sym0.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::T(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state7(__lookahead, __tokens, __sym1));
                }
                __Nonterminal::E(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state9(__lookahead, __tokens, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 6
    //   E = E "-" (*) T [EOF]
    //   E = E "-" (*) T ["-"]
    //   T = (*) "(" E ")" [EOF]
    //   T = (*) "(" E ")" ["-"]
    //   T = (*) "Num" [EOF]
    //   T = (*) "Num" ["-"]
    //
    //   "Num" -> Shift(S4)
    //   "(" -> Shift(S5)
    //
    //   T -> S11
    pub fn __state6<
        __TOKENS: Iterator<Item=Tok>,
    >(
        mut __lookahead: Option<Tok>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<i32>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<Tok>, __Nonterminal<>), Option<Tok>>
    {
        let mut __result: (Option<Tok>, __Nonterminal<>);
        match __lookahead {
            Some(Tok::Num(__tok0)) => {
                let mut __sym2 = &mut Some((__tok0));
                let __lookahead = __tokens.next();
                __result = try!(__state4(__lookahead, __tokens, __sym2));
            }
            Some(__tok @ Tok::LParen(..)) => {
                let mut __sym2 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state5(__lookahead, __tokens, __sym2));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        while __sym1.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::T(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state11(__lookahead, __tokens, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 7
    //   E = T (*) [")"]
    //   E = T (*) ["-"]
    //
    //   ")" -> Reduce(E = T => ActionFn(3);)
    //   "-" -> Reduce(E = T => ActionFn(3);)
    //
    pub fn __state7<
        __TOKENS: Iterator<Item=Tok>,
    >(
        mut __lookahead: Option<Tok>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<i32>,
    ) -> Result<(Option<Tok>, __Nonterminal<>), Option<Tok>>
    {
        let mut __result: (Option<Tok>, __Nonterminal<>);
        match __lookahead {
            Some(Tok::RParen(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__actions::__action3(__sym0);
                return Ok((__lookahead, __Nonterminal::E(__nt)));
            }
            Some(Tok::Minus(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__actions::__action3(__sym0);
                return Ok((__lookahead, __Nonterminal::E(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
    }

    // State 8
    //   E = (*) E "-" T [")"]
    //   E = (*) E "-" T ["-"]
    //   E = (*) T [")"]
    //   E = (*) T ["-"]
    //   T = (*) "(" E ")" [")"]
    //   T = (*) "(" E ")" ["-"]
    //   T = "(" (*) E ")" [")"]
    //   T = "(" (*) E ")" ["-"]
    //   T = (*) "Num" [")"]
    //   T = (*) "Num" ["-"]
    //
    //   "Num" -> Shift(S10)
    //   "(" -> Shift(S8)
    //
    //   E -> S12
    //   T -> S7
    pub fn __state8<
        __TOKENS: Iterator<Item=Tok>,
    >(
        mut __lookahead: Option<Tok>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
    ) -> Result<(Option<Tok>, __Nonterminal<>), Option<Tok>>
    {
        let mut __result: (Option<Tok>, __Nonterminal<>);
        match __lookahead {
            Some(Tok::Num(__tok0)) => {
                let mut __sym1 = &mut Some((__tok0));
                let __lookahead = __tokens.next();
                __result = try!(__state10(__lookahead, __tokens, __sym1));
            }
            Some(__tok @ Tok::LParen(..)) => {
                let mut __sym1 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state8(__lookahead, __tokens, __sym1));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        while __sym0.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::E(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state12(__lookahead, __tokens, __sym0, __sym1));
                }
                __Nonterminal::T(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state7(__lookahead, __tokens, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 9
    //   E = E (*) "-" T [")"]
    //   E = E (*) "-" T ["-"]
    //   T = "(" E (*) ")" [EOF]
    //   T = "(" E (*) ")" ["-"]
    //
    //   "-" -> Shift(S14)
    //   ")" -> Shift(S13)
    //
    pub fn __state9<
        __TOKENS: Iterator<Item=Tok>,
    >(
        mut __lookahead: Option<Tok>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
        __sym1: &mut Option<i32>,
    ) -> Result<(Option<Tok>, __Nonterminal<>), Option<Tok>>
    {
        let mut __result: (Option<Tok>, __Nonterminal<>);
        match __lookahead {
            Some(__tok @ Tok::Minus(..)) => {
                let mut __sym2 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state14(__lookahead, __tokens, __sym1, __sym2));
            }
            Some(__tok @ Tok::RParen(..)) => {
                let mut __sym2 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state13(__lookahead, __tokens, __sym0, __sym1, __sym2));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        return Ok(__result);
    }

    // State 10
    //   T = "Num" (*) [")"]
    //   T = "Num" (*) ["-"]
    //
    //   ")" -> Reduce(T = "Num" => ActionFn(4);)
    //   "-" -> Reduce(T = "Num" => ActionFn(4);)
    //
    pub fn __state10<
        __TOKENS: Iterator<Item=Tok>,
    >(
        mut __lookahead: Option<Tok>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<i32>,
    ) -> Result<(Option<Tok>, __Nonterminal<>), Option<Tok>>
    {
        let mut __result: (Option<Tok>, __Nonterminal<>);
        match __lookahead {
            Some(Tok::RParen(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__actions::__action4(__sym0);
                return Ok((__lookahead, __Nonterminal::T(__nt)));
            }
            Some(Tok::Minus(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__actions::__action4(__sym0);
                return Ok((__lookahead, __Nonterminal::T(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
    }

    // State 11
    //   E = E "-" T (*) [EOF]
    //   E = E "-" T (*) ["-"]
    //
    //   "-" -> Reduce(E = E, "-", T => ActionFn(2);)
    //   EOF -> Reduce(E = E, "-", T => ActionFn(2);)
    //
    pub fn __state11<
        __TOKENS: Iterator<Item=Tok>,
    >(
        mut __lookahead: Option<Tok>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<i32>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<i32>,
    ) -> Result<(Option<Tok>, __Nonterminal<>), Option<Tok>>
    {
        let mut __result: (Option<Tok>, __Nonterminal<>);
        match __lookahead {
            Some(Tok::Minus(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action2(__sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::E(__nt)));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action2(__sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::E(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
    }

    // State 12
    //   E = E (*) "-" T [")"]
    //   E = E (*) "-" T ["-"]
    //   T = "(" E (*) ")" [")"]
    //   T = "(" E (*) ")" ["-"]
    //
    //   ")" -> Shift(S15)
    //   "-" -> Shift(S14)
    //
    pub fn __state12<
        __TOKENS: Iterator<Item=Tok>,
    >(
        mut __lookahead: Option<Tok>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
        __sym1: &mut Option<i32>,
    ) -> Result<(Option<Tok>, __Nonterminal<>), Option<Tok>>
    {
        let mut __result: (Option<Tok>, __Nonterminal<>);
        match __lookahead {
            Some(__tok @ Tok::RParen(..)) => {
                let mut __sym2 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state15(__lookahead, __tokens, __sym0, __sym1, __sym2));
            }
            Some(__tok @ Tok::Minus(..)) => {
                let mut __sym2 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state14(__lookahead, __tokens, __sym1, __sym2));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        return Ok(__result);
    }

    // State 13
    //   T = "(" E ")" (*) [EOF]
    //   T = "(" E ")" (*) ["-"]
    //
    //   "-" -> Reduce(T = "(", E, ")" => ActionFn(5);)
    //   EOF -> Reduce(T = "(", E, ")" => ActionFn(5);)
    //
    pub fn __state13<
        __TOKENS: Iterator<Item=Tok>,
    >(
        mut __lookahead: Option<Tok>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
        __sym1: &mut Option<i32>,
        __sym2: &mut Option<Tok>,
    ) -> Result<(Option<Tok>, __Nonterminal<>), Option<Tok>>
    {
        let mut __result: (Option<Tok>, __Nonterminal<>);
        match __lookahead {
            Some(Tok::Minus(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action5(__sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::T(__nt)));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action5(__sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::T(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
    }

    // State 14
    //   E = E "-" (*) T [")"]
    //   E = E "-" (*) T ["-"]
    //   T = (*) "(" E ")" [")"]
    //   T = (*) "(" E ")" ["-"]
    //   T = (*) "Num" [")"]
    //   T = (*) "Num" ["-"]
    //
    //   "Num" -> Shift(S10)
    //   "(" -> Shift(S8)
    //
    //   T -> S16
    pub fn __state14<
        __TOKENS: Iterator<Item=Tok>,
    >(
        mut __lookahead: Option<Tok>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<i32>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<Tok>, __Nonterminal<>), Option<Tok>>
    {
        let mut __result: (Option<Tok>, __Nonterminal<>);
        match __lookahead {
            Some(Tok::Num(__tok0)) => {
                let mut __sym2 = &mut Some((__tok0));
                let __lookahead = __tokens.next();
                __result = try!(__state10(__lookahead, __tokens, __sym2));
            }
            Some(__tok @ Tok::LParen(..)) => {
                let mut __sym2 = &mut Some(__tok);
                let __lookahead = __tokens.next();
                __result = try!(__state8(__lookahead, __tokens, __sym2));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        while __sym1.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::T(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state16(__lookahead, __tokens, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 15
    //   T = "(" E ")" (*) [")"]
    //   T = "(" E ")" (*) ["-"]
    //
    //   ")" -> Reduce(T = "(", E, ")" => ActionFn(5);)
    //   "-" -> Reduce(T = "(", E, ")" => ActionFn(5);)
    //
    pub fn __state15<
        __TOKENS: Iterator<Item=Tok>,
    >(
        mut __lookahead: Option<Tok>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
        __sym1: &mut Option<i32>,
        __sym2: &mut Option<Tok>,
    ) -> Result<(Option<Tok>, __Nonterminal<>), Option<Tok>>
    {
        let mut __result: (Option<Tok>, __Nonterminal<>);
        match __lookahead {
            Some(Tok::RParen(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action5(__sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::T(__nt)));
            }
            Some(Tok::Minus(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action5(__sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::T(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
    }

    // State 16
    //   E = E "-" T (*) [")"]
    //   E = E "-" T (*) ["-"]
    //
    //   "-" -> Reduce(E = E, "-", T => ActionFn(2);)
    //   ")" -> Reduce(E = E, "-", T => ActionFn(2);)
    //
    pub fn __state16<
        __TOKENS: Iterator<Item=Tok>,
    >(
        mut __lookahead: Option<Tok>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<i32>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<i32>,
    ) -> Result<(Option<Tok>, __Nonterminal<>), Option<Tok>>
    {
        let mut __result: (Option<Tok>, __Nonterminal<>);
        match __lookahead {
            Some(Tok::Minus(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action2(__sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::E(__nt)));
            }
            Some(Tok::RParen(..)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__actions::__action2(__sym0, __sym1, __sym2);
                return Ok((__lookahead, __Nonterminal::E(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
    }
}

mod __actions {
    use util::tok::Tok;


    pub fn __action0<
    >(
        __0: i32,
    ) -> i32
    {
        (__0)
    }

    pub fn __action1<
    >(
        __0: i32,
    ) -> i32
    {
        (__0)
    }

    pub fn __action2<
    >(
        l: i32,
        _: Tok,
        r: i32,
    ) -> i32
    {
        l-r
    }

    pub fn __action3<
    >(
        __0: i32,
    ) -> i32
    {
        (__0)
    }

    pub fn __action4<
    >(
        __0: i32,
    ) -> i32
    {
        (__0)
    }

    pub fn __action5<
    >(
        _: Tok,
        __0: i32,
        _: Tok,
    ) -> i32
    {
        (__0)
    }
}
