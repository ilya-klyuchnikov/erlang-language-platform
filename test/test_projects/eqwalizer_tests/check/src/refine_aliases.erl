%%% Copyright (c) Meta Platforms, Inc. and affiliates. All rights reserved.
%%%
%%% This source code is licensed under the Apache 2.0 license found in
%%% the LICENSE file in the root directory of this source tree.

-module(refine_aliases).

-compile([export_all, nowarn_export_all]).

%% Demonstrates a bug in meetAux where parameterized type
%% aliases on the right side were not unfolded, causing
%% false overlaps in overloaded spec resolution.

-record(client, {pid :: pid()}).
-record(txn, {ref :: reference()}).

-type handler(T) :: fun((#client{}, binary()) -> T).
-type txn_handler(T) :: fun((#txn{}, binary()) -> T).

%% A completely unrelated parameterized fun type
-type callback(T) :: fun((atom(), atom(), atom(), atom()) -> T).

-spec get_handler(boolean()) ->
    handler(ok) | txn_handler(ok).
get_handler(true) -> fun(#client{}, _Req) -> ok end;
get_handler(false) -> fun(#txn{}, _Req) -> ok end.

%% Overloaded spec with generic type vars.
%% Overload 1: accepts handler(T) (arity-2 fun with #client{})
%% Overload 2: accepts callback(T) (arity-4 fun, completely different)
%%
%% Only overload 1 should match handler(ok) | txn_handler(ok).
%% But meet(txn_handler(ok), callback(T)) incorrectly returns
%% txn_handler(ok) because callback(T) has a type parameter,
%% making mayOverlap return true. Both overloads selected ->
%% DynamicType fallback -> no checking.
-spec execute
    (#client{}, handler(T), atom()) -> T;
    (#client{}, callback(T), atom()) -> T.
execute(_C, Fun, _Opts) when is_function(Fun, 4) ->
    Fun(a, b, c, d);
execute(C, Fun, _Opts) ->
    Fun(C, <<>>).

%% This should error: handler(ok) | txn_handler(ok) is not
%% a subtype of handler(T), because txn_handler(ok) has
%% #txn{} as first arg, not #client{}.
-spec use_neg(boolean()) -> ok.
use_neg(Flag) ->
    H = get_handler(Flag),
    execute(#client{pid = self()}, H, opts).
