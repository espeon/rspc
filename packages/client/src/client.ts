import {
  HasAnyLinkFlags,
  HasLinkFlags,
  JoinLinkFlags,
  LinkFlag,
  LinkFlags,
  Operation,
  ProceduresDef,
} from ".";

/**
 * TODO
 *
 * @internal
 */
export interface LinkOperation<T extends ProceduresDef> extends Operation {
  // next();
}

/**
 * TODO
 *
 * @internal
 */
export type Link<T extends ProceduresDef, TT, TFlag extends LinkFlag> = (
  p: LinkOperation<T>
) => LinkResponse<TT, TFlag>;

/**
 * TODO
 * TODO: Should this be marked as internal because it will be required to make a custom link.
 *
 * @internal
 */
export type LinkResponse<T, TFlag extends LinkFlag> = any; // TODO: Not `any`

/**
 * TODO
 */
export interface InitRspcOpts {
  /**
   * TODO
   */
  // onError(); // TODO: Make this work with links!
}

/**
 * TODO
 */
export function initRspc<T extends ProceduresDef>(
  opts?: InitRspcOpts
): Rspc<T> {
  return initRspcInner<T>();
}

// TODO: Utils
type And<T, U> = T extends true ? (U extends true ? true : false) : false;
type Or<T, U> = T extends true ? true : U extends true ? true : false;
type Not<T> = T extends true ? false : true;

// type LinkArg<
//   T extends ProceduresDef,
//   TT extends ProceduresDef,
//   TFlags extends LinkFlags,
//   TNewFlag extends LinkFlag
// > = And<
//   HasLinkFlags<TFlags, "built">,
//   Not<HasLinkFlags<TFlags, "subscriptionsUnsupported">>
// > extends true
//   ? "subscriptionsUnsupported" extends TNewFlag
//     ? "You must provide a link which supports subscriptions!"
//     : Link<T, TT, TNewFlag>
//   : Link<T, TT, TNewFlag>;

type LinkArg<
  T extends ProceduresDef,
  TT extends ProceduresDef,
  TFlags extends LinkFlags,
  TNewFlag extends LinkFlag
> = And<
  HasLinkFlags<TFlags, "built">,
  Not<HasLinkFlags<TFlags, "subscriptionsUnsupported">>
> extends true
  ? "subscriptionsUnsupported" extends TNewFlag
    ? "You must provide a link which supports subscriptions!"
    : Link<T, TT, TNewFlag>
  : Link<T, TT, TNewFlag>;

type UseFn<T extends ProceduresDef, TFlags extends LinkFlags> = HasAnyLinkFlags<
  TFlags,
  "terminatedLink"
> extends false
  ? {
      use<TT extends ProceduresDef, TNewFlag extends LinkFlag>(
        link: LinkArg<T, TT, TFlags, TNewFlag>
      ): Rspc<TT, JoinLinkFlags<TFlags, TNewFlag>>;
    }
  : {};

type BuildFn<T extends ProceduresDef, TFlags extends LinkFlags> = Or<
  HasAnyLinkFlags<TFlags, "built">,
  HasAnyLinkFlags<TFlags, "terminatedLink">
> extends false
  ? {
      build<TSupportSubscriptions extends boolean>(opts: {
        supportsSubscriptions: TSupportSubscriptions;
      }): Rspc<
        T,
        TFlags & {
          built: true;
        } & (TSupportSubscriptions extends false
            ? { subscriptionsUnsupported: true }
            : {})
      >;
    }
  : {};

type OperationFns<
  T extends ProceduresDef,
  TFlags extends LinkFlags
> = HasAnyLinkFlags<TFlags, "terminatedLink" | "built"> extends true
  ? {
      query<K extends T extends { key: string } ? T["key"] : never>(
        key: K
      ): void;
      mutate<K extends T extends { key: string } ? T["key"] : never>(
        key: K
      ): void;
      // TODO: `getQueryKey`
    } & (HasAnyLinkFlags<TFlags, "subscriptionsUnsupported"> extends true
      ? {}
      : { subscribe(): void })
  : {};

/**
 * The type of the rspc instance. This type is what powers all of the advanced type checking.
 *
 * @internal
 */
export type Rspc<
  T extends ProceduresDef,
  TFlags extends LinkFlags = {}
> = UseFn<T, TFlags> & OperationFns<T, TFlags> & BuildFn<T, TFlags>;

function initRspcInner<
  T extends ProceduresDef,
  TFlag extends LinkFlags = {}
>(): Rspc<T, TFlag> {
  return {
    // @ts-expect-error: TODO: Fix this. It's because of the discriminated union.
    use<TT extends ProceduresDef, TNewFlag extends Flag>(
      mw: Link<T, TT, TNewFlag>
    ): Rspc<TT, JoinLinkFlags<TFlag, TNewFlag>> {
      return initRspcInner<TT, JoinLinkFlags<TFlag, TNewFlag>>();
    },
    // @ts-expect-error: TODO: Fix this. It's because of the discriminated union.
    query<K extends T extends { key: string } ? T["key"] : never>(key: K) {
      // TODO
    },
    subscribe() {
      // TODO
    },
  } satisfies Rspc<T, {} /* TODO: Should be default? */> as any;
}
