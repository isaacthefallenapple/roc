use crate::def::Def;
use crate::expr::{AnnotatedMark, ClosureData, Expr::*};
use crate::expr::{Expr, Recursive};

use crate::pattern::Pattern;
use roc_collections::all::SendMap;
use roc_module::ident::TagName;
use roc_module::low_level::LowLevel;
use roc_module::symbol::Symbol;
use roc_region::all::{Loc, Region};
use roc_types::subs::{VarStore, Variable};

/// We use a rust macro to ensure that every LowLevel gets handled
macro_rules! map_symbol_to_lowlevel_and_arity {
    ($($lowlevel:ident; $symbol:ident; $number_of_args:literal),* $(,)?) => {
        fn def_for_symbol(symbol: Symbol, var_store: &mut VarStore) -> Option<Def> {
            // expands to a big (but non-exhaustive) match on symbols and maps them to a def
            // usually this means wrapping a lowlevel in a `Def` with the right number of
            // arguments (see the big enumeration below). In this match we have a bunch of cases
            // where that default strategy does not work.
            match symbol {
                $(
                Symbol::$symbol => Some((lowlevel_n($number_of_args))(Symbol::$symbol, LowLevel::$lowlevel, var_store)),
                )*

                Symbol::NUM_TO_I8 => Some(lowlevel_1(Symbol::NUM_TO_I8, LowLevel::NumIntCast, var_store)),
                Symbol::NUM_TO_I16 => Some(lowlevel_1(Symbol::NUM_TO_I16, LowLevel::NumIntCast, var_store)),
                Symbol::NUM_TO_I32 => Some(lowlevel_1(Symbol::NUM_TO_I32, LowLevel::NumIntCast, var_store)),
                Symbol::NUM_TO_I64 => Some(lowlevel_1(Symbol::NUM_TO_I64, LowLevel::NumIntCast, var_store)),
                Symbol::NUM_TO_I128 => Some(lowlevel_1(Symbol::NUM_TO_I128, LowLevel::NumIntCast, var_store)),
                Symbol::NUM_TO_U8 => Some(lowlevel_1(Symbol::NUM_TO_U8, LowLevel::NumIntCast, var_store)),
                Symbol::NUM_TO_U16 => Some(lowlevel_1(Symbol::NUM_TO_U16, LowLevel::NumIntCast, var_store)),
                Symbol::NUM_TO_U32 => Some(lowlevel_1(Symbol::NUM_TO_U32, LowLevel::NumIntCast, var_store)),
                Symbol::NUM_TO_U64 => Some(lowlevel_1(Symbol::NUM_TO_U64, LowLevel::NumIntCast, var_store)),
                Symbol::NUM_TO_U128 => Some(lowlevel_1(Symbol::NUM_TO_U128, LowLevel::NumIntCast, var_store)),
                Symbol::NUM_TO_NAT => Some(lowlevel_1(Symbol::NUM_TO_NAT, LowLevel::NumIntCast, var_store)),

                Symbol::NUM_INT_CAST => Some(lowlevel_1(Symbol::NUM_INT_CAST, LowLevel::NumIntCast, var_store)),

                Symbol::NUM_TO_F32 => Some(lowlevel_1(Symbol::NUM_TO_F32, LowLevel::NumToFloatCast, var_store)),
                Symbol::NUM_TO_F64 => Some(lowlevel_1(Symbol::NUM_TO_F64, LowLevel::NumToFloatCast, var_store)),

                Symbol::NUM_TO_I8_CHECKED => Some(to_num_checked(Symbol::NUM_TO_I8_CHECKED, var_store, LowLevel::NumToIntChecked)),
                Symbol::NUM_TO_I16_CHECKED => Some(to_num_checked(Symbol::NUM_TO_I16_CHECKED, var_store, LowLevel::NumToIntChecked)),
                Symbol::NUM_TO_I32_CHECKED => Some(to_num_checked(Symbol::NUM_TO_I32_CHECKED, var_store, LowLevel::NumToIntChecked)),
                Symbol::NUM_TO_I64_CHECKED => Some(to_num_checked(Symbol::NUM_TO_I64_CHECKED, var_store, LowLevel::NumToIntChecked)),
                Symbol::NUM_TO_I128_CHECKED => Some(to_num_checked(Symbol::NUM_TO_I128_CHECKED, var_store, LowLevel::NumToIntChecked)),
                Symbol::NUM_TO_U8_CHECKED => Some(to_num_checked(Symbol::NUM_TO_U8_CHECKED, var_store, LowLevel::NumToIntChecked)),
                Symbol::NUM_TO_U16_CHECKED => Some(to_num_checked(Symbol::NUM_TO_U16_CHECKED, var_store, LowLevel::NumToIntChecked)),
                Symbol::NUM_TO_U32_CHECKED => Some(to_num_checked(Symbol::NUM_TO_U32_CHECKED, var_store, LowLevel::NumToIntChecked)),
                Symbol::NUM_TO_U64_CHECKED => Some(to_num_checked(Symbol::NUM_TO_U64_CHECKED, var_store, LowLevel::NumToIntChecked)),
                Symbol::NUM_TO_U128_CHECKED => Some(to_num_checked(Symbol::NUM_TO_U128_CHECKED, var_store, LowLevel::NumToIntChecked)),
                Symbol::NUM_TO_NAT_CHECKED => Some(to_num_checked(Symbol::NUM_TO_NAT_CHECKED, var_store, LowLevel::NumToIntChecked)),

                Symbol::NUM_TO_F32_CHECKED => Some(to_num_checked(Symbol::NUM_TO_F32_CHECKED, var_store, LowLevel::NumToFloatChecked)),
                Symbol::NUM_TO_F64_CHECKED => Some(to_num_checked(Symbol::NUM_TO_F64_CHECKED, var_store, LowLevel::NumToFloatChecked)),

                Symbol::NUM_DIV_FRAC => Some(lowlevel_2(Symbol::NUM_DIV_FRAC, LowLevel::NumDivUnchecked, var_store)),
                Symbol::NUM_DIV_TRUNC => Some(lowlevel_2(Symbol::NUM_DIV_TRUNC, LowLevel::NumDivUnchecked, var_store)),

                Symbol::DICT_EMPTY => Some(dict_empty(Symbol::DICT_EMPTY, var_store)),

                Symbol::SET_UNION => Some(lowlevel_2(Symbol::SET_UNION, LowLevel::DictUnion, var_store)),
                Symbol::SET_DIFFERENCE => Some(lowlevel_2(Symbol::SET_DIFFERENCE, LowLevel::DictDifference, var_store)),
                Symbol::SET_INTERSECTION => Some(lowlevel_2(Symbol::SET_INTERSECTION, LowLevel::DictIntersection, var_store)),

                Symbol::SET_TO_LIST => Some(lowlevel_1(Symbol::SET_TO_LIST, LowLevel::DictKeys, var_store)),
                Symbol::SET_REMOVE => Some(lowlevel_2(Symbol::SET_REMOVE, LowLevel::DictRemove, var_store)),
                Symbol::SET_INSERT => Some(set_insert(Symbol::SET_INSERT, var_store)),
                Symbol::SET_EMPTY => Some(set_empty(Symbol::SET_EMPTY, var_store)),
                Symbol::SET_SINGLE => Some(set_single(Symbol::SET_SINGLE, var_store)),

                _ => None,
            }
        }

        fn _enforce_exhaustiveness(lowlevel: LowLevel) -> Symbol {
            // when adding a new lowlevel, this match will stop being exhaustive, and give a
            // compiler error. Most likely, you are adding a new lowlevel that maps directly to a
            // symbol. For instance, you want to have `List.foo` to stand for the `ListFoo`
            // lowlevel. In that case, see below in the invocation of `map_symbol_to_lowlevel_and_arity!`
            //
            // Below, we explicitly handle some exceptions to the pattern where a lowlevel maps
            // directly to a symbol. If you are unsure if your lowlevel is an exception, assume
            // that it isn't and just see if that works.
            match lowlevel {
                $(
                LowLevel::$lowlevel => Symbol::$symbol,
                )*

                // these are implemented explicitly in for_symbol because they are polymorphic
                LowLevel::NumIntCast => unreachable!(),
                LowLevel::NumToFloatCast => unreachable!(),
                LowLevel::NumToIntChecked => unreachable!(),
                LowLevel::NumToFloatChecked => unreachable!(),
                LowLevel::NumDivUnchecked => unreachable!(),
                LowLevel::DictEmpty => unreachable!(),

                // these are used internally and not tied to a symbol
                LowLevel::Hash => unimplemented!(),
                LowLevel::PtrCast => unimplemented!(),
                LowLevel::RefCountInc => unimplemented!(),
                LowLevel::RefCountDec => unimplemented!(),

                // these are not implemented, not sure why
                LowLevel::StrFromInt => unimplemented!(),
                LowLevel::StrFromFloat => unimplemented!(),
                LowLevel::NumIsFinite => unimplemented!(),
            }
        }
    };
}

// here is where we actually specify the mapping for the fast majority of cases that follow the
// pattern of a symbol mapping directly to a lowlevel. In other words, most lowlevels (left) are generated
// by only one specific symbol (center). We also specify the arity (number of arguments) of the lowlevel (right)
map_symbol_to_lowlevel_and_arity! {
    StrConcat; STR_CONCAT; 2,
    StrJoinWith; STR_JOIN_WITH; 2,
    StrIsEmpty; STR_IS_EMPTY; 1,
    StrStartsWith; STR_STARTS_WITH; 2,
    StrStartsWithScalar; STR_STARTS_WITH_SCALAR; 2,
    StrEndsWith; STR_ENDS_WITH; 2,
    StrSplit; STR_SPLIT; 2,
    StrCountGraphemes; STR_COUNT_GRAPHEMES; 1,
    StrCountUtf8Bytes; STR_COUNT_UTF8_BYTES; 1,
    StrFromUtf8Range; STR_FROM_UTF8_RANGE_LOWLEVEL; 3,
    StrToUtf8; STR_TO_UTF8; 1,
    StrRepeat; STR_REPEAT; 2,
    StrTrim; STR_TRIM; 1,
    StrTrimLeft; STR_TRIM_LEFT; 1,
    StrTrimRight; STR_TRIM_RIGHT; 1,
    StrToScalars; STR_TO_SCALARS; 1,
    StrGetUnsafe; STR_GET_UNSAFE; 2,
    StrSubstringUnsafe; STR_SUBSTRING_UNSAFE; 3,
    StrReserve; STR_RESERVE; 1,
    StrAppendScalar; STR_APPEND_SCALAR_UNSAFE; 2,
    StrGetScalarUnsafe; STR_GET_SCALAR_UNSAFE; 2,
    StrToNum; STR_TO_NUM; 1,

    ListLen; LIST_LEN; 1,
    ListWithCapacity; LIST_WITH_CAPACITY; 1,
    ListReserve; LIST_RESERVE; 2,
    ListIsUnique; LIST_IS_UNIQUE; 1,
    ListAppendUnsafe; LIST_APPEND_UNSAFE; 2,
    ListPrepend; LIST_PREPEND; 2,
    ListGetUnsafe; LIST_GET_UNSAFE; 2,
    ListReplaceUnsafe; LIST_REPLACE_UNSAFE; 3,
    ListConcat; LIST_CONCAT; 2,
    ListMap; LIST_MAP; 2,
    ListMap2; LIST_MAP2; 3,
    ListMap3; LIST_MAP3; 4,
    ListMap4; LIST_MAP4; 5,
    ListSortWith; LIST_SORT_WITH; 2,
    ListSublist; LIST_SUBLIST_LOWLEVEL; 3,
    ListDropAt; LIST_DROP_AT; 2,
    ListSwap; LIST_SWAP; 3,

    DictSize; DICT_LEN; 1,
    DictInsert; DICT_INSERT; 3,
    DictRemove; DICT_REMOVE; 2,
    DictContains; DICT_CONTAINS; 2,
    DictGetUnsafe; DICT_GET_LOWLEVEL; 2,
    DictKeys; DICT_KEYS; 1,
    DictValues; DICT_VALUES; 1,
    DictUnion; DICT_UNION; 2,
    DictIntersection; DICT_INTERSECTION; 2,
    DictDifference; DICT_DIFFERENCE; 2,
    DictWalk; DICT_WALK; 3,

    SetFromList; SET_FROM_LIST; 1,
    SetToDict; SET_TO_DICT; 1,

    NumAdd; NUM_ADD; 2,
    NumAddWrap; NUM_ADD_WRAP; 2,
    NumAddChecked; NUM_ADD_CHECKED_LOWLEVEL; 2,
    NumAddSaturated; NUM_ADD_SATURATED; 2,
    NumSub; NUM_SUB; 2,
    NumSubWrap; NUM_SUB_WRAP; 2,
    NumSubChecked; NUM_SUB_CHECKED_LOWLEVEL; 2,
    NumSubSaturated; NUM_SUB_SATURATED; 2,
    NumMul; NUM_MUL; 2,
    NumMulWrap; NUM_MUL_WRAP; 2,
    NumMulSaturated; NUM_MUL_SATURATED; 2,
    NumMulChecked; NUM_MUL_CHECKED_LOWLEVEL; 2,
    NumGt; NUM_GT; 2,
    NumGte; NUM_GTE; 2,
    NumLt; NUM_LT; 2,
    NumLte; NUM_LTE; 2,
    NumCompare; NUM_COMPARE; 2,
    NumDivCeilUnchecked; NUM_DIV_CEIL; 2,
    NumRemUnchecked; NUM_REM; 2,
    NumIsMultipleOf; NUM_IS_MULTIPLE_OF; 2,
    NumAbs; NUM_ABS; 1,
    NumNeg; NUM_NEG; 1,
    NumSin; NUM_SIN; 1,
    NumCos; NUM_COS; 1,
    NumSqrtUnchecked; NUM_SQRT; 1,
    NumLogUnchecked; NUM_LOG; 1,
    NumRound; NUM_ROUND; 1,
    NumToFrac; NUM_TO_FRAC; 1,
    NumPow; NUM_POW; 2,
    NumCeiling; NUM_CEILING; 1,
    NumPowInt; NUM_POW_INT; 2,
    NumFloor; NUM_FLOOR; 1,
    NumAtan; NUM_ATAN; 1,
    NumAcos; NUM_ACOS; 1,
    NumAsin; NUM_ASIN; 1,
    NumBytesToU16; NUM_BYTES_TO_U16_LOWLEVEL; 2,
    NumBytesToU32; NUM_BYTES_TO_U32_LOWLEVEL; 2,
    NumBitwiseAnd; NUM_BITWISE_AND; 2,
    NumBitwiseXor; NUM_BITWISE_XOR; 2,
    NumBitwiseOr; NUM_BITWISE_OR; 2,
    NumShiftLeftBy; NUM_SHIFT_LEFT; 2,
    NumShiftRightBy; NUM_SHIFT_RIGHT; 2,
    NumShiftRightZfBy; NUM_SHIFT_RIGHT_ZERO_FILL; 2,
    NumToStr; NUM_TO_STR; 1,

    Eq; BOOL_EQ; 2,
    NotEq; BOOL_NEQ; 2,
    And; BOOL_AND; 2,
    Or; BOOL_OR; 2,
    Not; BOOL_NOT; 1,
    BoxExpr; BOX_BOX_FUNCTION; 1,
    UnboxExpr; BOX_UNBOX; 1,
    Unreachable; LIST_UNREACHABLE; 1,
}

/// Some builtins cannot be constructed in code gen alone, and need to be defined
/// as separate Roc defs. For example, List.get has this type:
///
/// List.get : List elem, Nat -> Result elem [OutOfBounds]*
///
/// Because this returns an open tag union for its Err type, it's not possible
/// for code gen to return a hardcoded value for OutOfBounds. For example,
/// if this Result unifies to [Foo, OutOfBounds] then OutOfBOunds will
/// get assigned the number 1 (because Foo got 0 alphabetically), whereas
/// if it unifies to [OutOfBounds, Qux] then OutOfBounds will get the number 0.
///
/// Getting these numbers right requires having List.get participate in the
/// normal type-checking and monomorphization processes. As such, this function
/// returns a normal def for List.get, which performs a bounds check and then
/// delegates to the compiler-internal List.getUnsafe function to do the actual
/// lookup (if the bounds check passed). That internal function is hardcoded in code gen,
/// which works fine because it doesn't involve any open tag unions.

/// Does a builtin depend on any other builtins?
///
/// NOTE: you are supposed to give all symbols that are relied on,
/// even those that are relied on transitively!
pub fn builtin_dependencies(symbol: Symbol) -> &'static [Symbol] {
    match symbol {
        Symbol::LIST_SORT_ASC => &[Symbol::LIST_SORT_WITH, Symbol::NUM_COMPARE],
        Symbol::LIST_SORT_DESC => &[Symbol::LIST_SORT_WITH],
        Symbol::LIST_PRODUCT => &[Symbol::LIST_WALK, Symbol::NUM_MUL],
        Symbol::LIST_SUM => &[Symbol::LIST_WALK, Symbol::NUM_ADD],
        Symbol::LIST_SET => &[Symbol::LIST_REPLACE],
        _ => &[],
    }
}

/// Implementation for a builtin
pub fn builtin_defs_map(symbol: Symbol, var_store: &mut VarStore) -> Option<Def> {
    debug_assert!(symbol.is_builtin());

    def_for_symbol(symbol, var_store)
}

fn lowlevel_n(n: usize) -> fn(Symbol, LowLevel, &mut VarStore) -> Def {
    match n {
        0 => unimplemented!(),
        1 => lowlevel_1,
        2 => lowlevel_2,
        3 => lowlevel_3,
        4 => lowlevel_4,
        5 => lowlevel_5,
        _ => unimplemented!(),
    }
}

fn lowlevel_1(symbol: Symbol, op: LowLevel, var_store: &mut VarStore) -> Def {
    let arg1_var = var_store.fresh();
    let ret_var = var_store.fresh();

    let body = RunLowLevel {
        op,
        args: vec![(arg1_var, Var(Symbol::ARG_1))],
        ret_var,
    };

    defn(
        symbol,
        vec![(arg1_var, Symbol::ARG_1)],
        var_store,
        body,
        ret_var,
    )
}

fn lowlevel_2(symbol: Symbol, op: LowLevel, var_store: &mut VarStore) -> Def {
    let arg1_var = var_store.fresh();
    let arg2_var = var_store.fresh();
    let ret_var = var_store.fresh();

    let body = RunLowLevel {
        op,
        args: vec![
            (arg1_var, Var(Symbol::ARG_1)),
            (arg2_var, Var(Symbol::ARG_2)),
        ],
        ret_var,
    };

    defn(
        symbol,
        vec![(arg1_var, Symbol::ARG_1), (arg2_var, Symbol::ARG_2)],
        var_store,
        body,
        ret_var,
    )
}

fn lowlevel_3(symbol: Symbol, op: LowLevel, var_store: &mut VarStore) -> Def {
    let arg1_var = var_store.fresh();
    let arg2_var = var_store.fresh();
    let arg3_var = var_store.fresh();
    let ret_var = var_store.fresh();

    let body = RunLowLevel {
        op,
        args: vec![
            (arg1_var, Var(Symbol::ARG_1)),
            (arg2_var, Var(Symbol::ARG_2)),
            (arg3_var, Var(Symbol::ARG_3)),
        ],
        ret_var,
    };

    defn(
        symbol,
        vec![
            (arg1_var, Symbol::ARG_1),
            (arg2_var, Symbol::ARG_2),
            (arg3_var, Symbol::ARG_3),
        ],
        var_store,
        body,
        ret_var,
    )
}

fn lowlevel_4(symbol: Symbol, op: LowLevel, var_store: &mut VarStore) -> Def {
    let arg1_var = var_store.fresh();
    let arg2_var = var_store.fresh();
    let arg3_var = var_store.fresh();
    let arg4_var = var_store.fresh();
    let ret_var = var_store.fresh();

    let body = RunLowLevel {
        op,
        args: vec![
            (arg1_var, Var(Symbol::ARG_1)),
            (arg2_var, Var(Symbol::ARG_2)),
            (arg3_var, Var(Symbol::ARG_3)),
            (arg4_var, Var(Symbol::ARG_4)),
        ],
        ret_var,
    };

    defn(
        symbol,
        vec![
            (arg1_var, Symbol::ARG_1),
            (arg2_var, Symbol::ARG_2),
            (arg3_var, Symbol::ARG_3),
            (arg4_var, Symbol::ARG_4),
        ],
        var_store,
        body,
        ret_var,
    )
}

fn lowlevel_5(symbol: Symbol, op: LowLevel, var_store: &mut VarStore) -> Def {
    let arg1_var = var_store.fresh();
    let arg2_var = var_store.fresh();
    let arg3_var = var_store.fresh();
    let arg4_var = var_store.fresh();
    let arg5_var = var_store.fresh();
    let ret_var = var_store.fresh();

    let body = RunLowLevel {
        op,
        args: vec![
            (arg1_var, Var(Symbol::ARG_1)),
            (arg2_var, Var(Symbol::ARG_2)),
            (arg3_var, Var(Symbol::ARG_3)),
            (arg4_var, Var(Symbol::ARG_4)),
            (arg5_var, Var(Symbol::ARG_5)),
        ],
        ret_var,
    };

    defn(
        symbol,
        vec![
            (arg1_var, Symbol::ARG_1),
            (arg2_var, Symbol::ARG_2),
            (arg3_var, Symbol::ARG_3),
            (arg4_var, Symbol::ARG_4),
            (arg5_var, Symbol::ARG_5),
        ],
        var_store,
        body,
        ret_var,
    )
}

#[inline(always)]
fn defn(
    fn_name: Symbol,
    args: Vec<(Variable, Symbol)>,
    var_store: &mut VarStore,
    body: Expr,
    ret_var: Variable,
) -> Def {
    let expr = defn_help(fn_name, args, var_store, body, ret_var);

    Def {
        loc_pattern: Loc {
            region: Region::zero(),
            value: Pattern::Identifier(fn_name),
        },
        loc_expr: Loc {
            region: Region::zero(),
            value: expr,
        },
        expr_var: var_store.fresh(),
        pattern_vars: SendMap::default(),
        annotation: None,
    }
}

#[inline(always)]
fn defn_help(
    fn_name: Symbol,
    args: Vec<(Variable, Symbol)>,
    var_store: &mut VarStore,
    body: Expr,
    ret_var: Variable,
) -> Expr {
    use crate::pattern::Pattern::*;

    let closure_args = args
        .into_iter()
        .map(|(var, symbol)| {
            (
                var,
                AnnotatedMark::new(var_store),
                no_region(Identifier(symbol)),
            )
        })
        .collect();

    Closure(ClosureData {
        function_type: var_store.fresh(),
        closure_type: var_store.fresh(),
        return_type: ret_var,
        name: fn_name,
        captured_symbols: Vec::new(),
        recursive: Recursive::NotRecursive,
        arguments: closure_args,
        loc_body: Box::new(no_region(body)),
    })
}

#[inline(always)]
fn no_region<T>(value: T) -> Loc<T> {
    Loc {
        region: Region::zero(),
        value,
    }
}

#[inline(always)]
fn tag(name: &'static str, args: Vec<Expr>, var_store: &mut VarStore) -> Expr {
    Expr::Tag {
        variant_var: var_store.fresh(),
        ext_var: var_store.fresh(),
        name: TagName(name.into()),
        arguments: args
            .into_iter()
            .map(|expr| (var_store.fresh(), no_region(expr)))
            .collect::<Vec<(Variable, Loc<Expr>)>>(),
    }
}

fn to_num_checked(symbol: Symbol, var_store: &mut VarStore, lowlevel: LowLevel) -> Def {
    let bool_var = var_store.fresh();
    let num_var_1 = var_store.fresh();
    let num_var_2 = var_store.fresh();
    let ret_var = var_store.fresh();
    let record_var = var_store.fresh();

    // let arg_2 = RunLowLevel NumToXXXChecked arg_1
    // if arg_2.b then
    //   Err OutOfBounds
    // else
    //   Ok arg_2.a
    //
    // "a" and "b" because the lowlevel return value looks like { converted_val: XXX, out_of_bounds: bool },
    // and codegen will sort by alignment, so "a" will be the first key, etc.

    let cont = If {
        branch_var: ret_var,
        cond_var: bool_var,
        branches: vec![(
            // if-condition
            no_region(
                // arg_2.b
                Access {
                    record_var,
                    ext_var: var_store.fresh(),
                    field: "b".into(),
                    field_var: var_store.fresh(),
                    loc_expr: Box::new(no_region(Var(Symbol::ARG_2))),
                },
            ),
            // out of bounds!
            no_region(tag(
                "Err",
                vec![tag("OutOfBounds", Vec::new(), var_store)],
                var_store,
            )),
        )],
        final_else: Box::new(
            // all is well
            no_region(
                // Ok arg_2.a
                tag(
                    "Ok",
                    vec![
                        // arg_2.a
                        Access {
                            record_var,
                            ext_var: var_store.fresh(),
                            field: "a".into(),
                            field_var: num_var_2,
                            loc_expr: Box::new(no_region(Var(Symbol::ARG_2))),
                        },
                    ],
                    var_store,
                ),
            ),
        ),
    };

    // arg_2 = RunLowLevel NumToXXXChecked arg_1
    let def = crate::def::Def {
        loc_pattern: no_region(Pattern::Identifier(Symbol::ARG_2)),
        loc_expr: no_region(RunLowLevel {
            op: lowlevel,
            args: vec![(num_var_1, Var(Symbol::ARG_1))],
            ret_var: record_var,
        }),
        expr_var: record_var,
        pattern_vars: SendMap::default(),
        annotation: None,
    };

    let body = LetNonRec(Box::new(def), Box::new(no_region(cont)));

    defn(
        symbol,
        vec![(num_var_1, Symbol::ARG_1)],
        var_store,
        body,
        ret_var,
    )
}

/// Dict.empty : Dict * *
fn dict_empty(symbol: Symbol, var_store: &mut VarStore) -> Def {
    let dict_var = var_store.fresh();
    let body = RunLowLevel {
        op: LowLevel::DictEmpty,
        args: vec![],
        ret_var: dict_var,
    };

    Def {
        annotation: None,
        expr_var: dict_var,
        loc_expr: Loc::at_zero(body),
        loc_pattern: Loc::at_zero(Pattern::Identifier(symbol)),
        pattern_vars: SendMap::default(),
    }
}

/// Set.empty : Set *
fn set_empty(symbol: Symbol, var_store: &mut VarStore) -> Def {
    let set_var = var_store.fresh();
    let body = RunLowLevel {
        op: LowLevel::DictEmpty,
        args: vec![],
        ret_var: set_var,
    };

    Def {
        annotation: None,
        expr_var: set_var,
        loc_expr: Loc::at_zero(body),
        loc_pattern: Loc::at_zero(Pattern::Identifier(symbol)),
        pattern_vars: SendMap::default(),
    }
}

/// Set.insert : Set k, k -> Set k
fn set_insert(symbol: Symbol, var_store: &mut VarStore) -> Def {
    let dict_var = var_store.fresh();
    let key_var = var_store.fresh();
    let val_var = Variable::EMPTY_RECORD;

    let body = RunLowLevel {
        op: LowLevel::DictInsert,
        args: vec![
            (dict_var, Var(Symbol::ARG_1)),
            (key_var, Var(Symbol::ARG_2)),
            (val_var, EmptyRecord),
        ],
        ret_var: dict_var,
    };

    defn(
        symbol,
        vec![(dict_var, Symbol::ARG_1), (key_var, Symbol::ARG_2)],
        var_store,
        body,
        dict_var,
    )
}

/// Set.single : k -> Set k
fn set_single(symbol: Symbol, var_store: &mut VarStore) -> Def {
    let key_var = var_store.fresh();
    let set_var = var_store.fresh();
    let value_var = Variable::EMPTY_RECORD;

    let empty = RunLowLevel {
        op: LowLevel::DictEmpty,
        args: vec![],
        ret_var: set_var,
    };

    let body = RunLowLevel {
        op: LowLevel::DictInsert,
        args: vec![
            (set_var, empty),
            (key_var, Var(Symbol::ARG_1)),
            (value_var, EmptyRecord),
        ],
        ret_var: set_var,
    };

    defn(
        symbol,
        vec![(key_var, Symbol::ARG_1)],
        var_store,
        body,
        set_var,
    )
}