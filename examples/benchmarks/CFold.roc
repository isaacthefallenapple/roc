app "cfold"
    packages { base: "platform" }
    imports [base.Task]
    provides [ main ] to base

# adapted from https://github.com/koka-lang/koka/blob/master/test/bench/haskell/cfold.hs

main : Task.Task {} []
main =
    e = mkExpr 3 1
    unoptimized = eval e
    optimized = eval (constFolding (reassoc e))

    unoptimized
        |> Str.fromInt
        |> Str.concat " & "
        |> Str.concat (Str.fromInt optimized)
        |> Task.putLine

Expr : [
    Add Expr Expr,
    Mul Expr Expr,
    Val I64,
    Var I64
    ]

mkExpr : I64, I64 -> Expr
mkExpr = \n , v ->
    when n is
        0 -> if v == 0 then Var 1 else Val v
        _ -> Add (mkExpr (n-1) (v+1)) (mkExpr (n-1) (max (v-1) 0))

max : I64, I64 -> I64
max = \a, b -> if a > b then a else b


appendAdd : Expr, Expr -> Expr
appendAdd = \e1, e2 ->
    when e1 is
        Add a1 a2 -> Add a1 (appendAdd a2 e2)
        _ -> Add e1 e2

appendMul : Expr, Expr -> Expr
appendMul = \e1, e2 ->
    when e1 is
        Mul a1 a2 -> Mul a1 (appendMul a2 e2)
        _ -> Mul e1 e2


eval : Expr -> I64
eval = \e ->
    when e is
        Var _ -> 0
        Val v -> v
        Add l r -> eval l + eval r
        Mul l r -> eval l * eval r

reassoc : Expr -> Expr
reassoc = \e ->
    when e is
        Add e1 e2 ->
            x1 = reassoc e1
            x2 = reassoc e2

            appendAdd x1 x2

        Mul e1 e2 ->
            x1 = reassoc e1
            x2 = reassoc e2

            appendMul x1 x2

        _ -> e

constFolding : Expr -> Expr
constFolding = \e ->
    when e is
        Add e1 e2 ->
            x1 = constFolding e1
            x2 = constFolding e2

            when Pair x1 x2 is
                Pair (Val a) (Val b) -> Val (a+b)
                # Pair (Val a) (Add (Val b) x) -> Add (Val (a+b)) x
                Pair (Val a) (Add x (Val b)) -> Add (Val (a+b)) x
                Pair _ _                     -> Add x1 x2

        Mul e1 e2 ->
            x1 = constFolding e1
            x2 = constFolding e2
        
            when Pair x1 x2 is
                Pair (Val a) (Val b) -> Val (a*b)
                Pair (Val a) (Mul (Val b) x) -> Mul (Val (a*b)) x
                Pair (Val a) (Mul x (Val b)) -> Mul (Val (a*b)) x
                Pair _ _                     -> Mul x1 x2

        _ -> e
