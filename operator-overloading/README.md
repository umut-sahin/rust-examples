<div align="center">
  <h1>operator-overloading</h1>
  A really basic <a href="https://en.wikipedia.org/wiki/Complex_number">Complex Number</a> library to demonstrate overloading operators in <a href="https://www.rust-lang.org/">Rust</a>
</div>

## Authors

- [umut-sahin](https://github.com/umut-sahin) - Umut Şahin \<umutsahin@protonmail.com>

## Prerequisites

None

## Description

In Rust, operator overloading is achieved with traits.
So to overload operators for custom types, you implement the traits corresponding to the operators you are trying to overload.

Here are all the overloadable operators, and their desugared form:

First, let's imagine we have the following variables:
```
let mut lhs: T = ...;
let mut rhs: U = ...;
```

Now, when you write the **Regular form**, the compiler sees the **Desugared form**.

| Operation | Regular form | Desugared form | Output Type |
|-----------|--------------|----------------|-------------|
| Equality check | **lhs == rhs;** | **\<T as [**std::cmp::PartialEq**]\<U>>::[**eq**]\(&lhs, &rhs);** | [**bool**] |
| Inequality check | **lhs != rhs;** | **\<T as [**std::cmp::PartialEq**]\<U>>::[**ne**]\(&lhs, &rhs);** | [**bool**] |
| Grater than or equal to check | **lhs >= rhs;** | **\<T as [**std::cmp::PartialOrd**]\<U>>::[**ge**]\(&lhs, &rhs);** | [**bool**] |
| Grater than check | **lhs > rhs;** | **\<T as [**std::cmp::PartialOrd**]\<U>>::[**gt**]\(&lhs, &rhs);** | [**bool**] |
| Lower than or equal to check | **lhs <= rhs;** | **\<T as [**std::cmp::PartialOrd**]\<U>>::[**le**]\(&lhs, &rhs);** | [**bool**] |
| Lower than check | **lhs < rhs;** | **\<T as [**std::cmp::PartialOrd**]\<U>>::[**lt**]\(&lhs, &rhs);** | [**bool**] |
| Addition | **lhs + rhs;** | **\<T as [**std::ops::Add**]\<U>>::[**add**]\(lhs, rhs);** | \<T as [**std::ops::Add**]\<U>>::[**Output**](**std::ops::Add::Output**) |
| Addition in-place | **lhs += rhs;** | **\<T as [**std::ops::AddAssign**]\<U>>::[**add_assign**]\(&mut lhs, rhs);** | () |
| Bitwise AND | **lhs & rhs;** | **\<T as [**std::ops::BitAnd**]\<U>>::[**bitand**]\(lhs, rhs);** | \<T as [**std::ops::BitAnd**]\<U>>::[**Output**](**std::ops::BitAnd::Output**) |
| Bitwise AND in-place | **lhs &= rhs;** | **\<T as [**std::ops::BitAndAssign**]\<U>>::[**bitand_assign**]\(&mut lhs, rhs);** | () |
| Bitwise OR | **lhs &#124; rhs;** | **\<T as [**std::ops::BitOr**]\<U>>::[**bitor**]\(lhs, rhs);** | \<T as [**std::ops::BitOr**]\<U>>::[**Output**](**std::ops::BitOr::Output**) |
| Bitwise OR in-place | **lhs &#124;= rhs;** | **\<T as [**std::ops::BitOrAssign**]\<U>>::[**bitor_assign**]\(&mut lhs, rhs);** | () |
| Bitwise XOR | **lhs ^ rhs;** | **\<T as [**std::ops::BitXor**]\<U>>::[**bitxor**]\(lhs, rhs);** | \<T as [**std::ops::BitXor**]\<U>>::[**Output**](**std::ops::BitXor::Output**) |
| Bitwise XOR in-place | **lhs ^= rhs;** | **\<T as [**std::ops::BitXorAssign**]\<U>>::[**bitxor_assign**]\(&mut lhs, rhs);** | () |
| Immutable dereferencing | **\*lhs;** | **\<T as [**std::ops::Deref**]>::[**deref**]\(&lhs);** | &<T as [**std::ops::Deref**]>::[**Target**](**std::ops::Deref::Target**) |
| Mutable dereferencing | **\*lhs;** | **\<T as [**std::ops::DerefMut**]>::[**deref_mut**]\(&mut lhs);** | &mut <T as [**std::ops::DerefMut**]>::[**Target**](**std::ops::DerefMut::Target**) |
| Division | **lhs / rhs;** | **\<T as [**std::ops::Div**]\<U>>::[**div**]\(lhs, rhs);** | \<T as [**std::ops::Div**]\<U>>::[**Output**](**std::ops::Div::Output**) |
| Division in-place | **lhs /= rhs;** | **\<T as [**std::ops::DivAssign**]\<U>>::[**div_assign**]\(&mut lhs, rhs);** | () |
| Destruction | **drop(lhs)** | **\<T as [**std::ops::Drop**]>::[**drop**]\(&mut lhs);** | () |
| Immutable indexing | **lhs\[rhs]** | **\<T as [**std::ops::Index**]\<U>>::[**index**]\(&lhs, rhs);** | &\<T as [**std::ops::Index**]\<U>>::[**Output**](**std::ops::Index::Output**) |
| Mutable indexing | **lhs\[rhs]** | **\<T as [**std::ops::IndexMut**]\<U>>::[**index_mut**]\(&mut lhs, rhs);** | &mut \<T as [**std::ops::IndexMut**]\<U>>::[**Output**](**std::ops::IndexMut::Output**) |
| Multiplication | **lhs \* rhs;** | **\<T as [**std::ops::Mul**]\<U>>::[**mul**]\(lhs, rhs);** | \<T as [**std::ops::Mul**]\<U>>::[**Output**](**std::ops::Mul::Output**) |
| Multiplication in-place | **lhs \*= rhs;** | **\<T as [**std::ops::MulAssign**]\<U>>::[**mul_assign**]\(&mut lhs, rhs);** | () |
| Negation | **-lhs;** | **\<T as [**std::ops::Neg**]>::[**neg**]\(lhs);** | <T as [**std::ops::Neg**]>::[**Output**](**std::ops::Neg::Output**) |
| Logical negation | **!lhs;** | **\<T as [**std::ops::Not**]>::[**not**]\(lhs);** | <T as [**std::ops::Not**]>::[**Output**](**std::ops::Not::Output**) |
| Reminder | **lhs % rhs;** | **\<T as [**std::ops::Rem**]\<U>>::[**rem**]\(lhs, rhs);** | \<T as [**std::ops::Rem**]\<U>>::[**Output**](**std::ops::Rem::Output**) |
| Reminder in-place | **lhs %= rhs;** | **\<T as [**std::ops::RemAssign**]\<U>>::[**rem_assign**]\(&mut lhs, rhs);** | () |
| Shift left | **lhs << rhs;** | **\<T as [**std::ops::Shl**]\<U>>::[**shl**]\(lhs, rhs);** | \<T as [**std::ops::Shl**]\<U>>::[**Output**](**std::ops::Shl::Output**) |
| Shift left in-place | **lhs <<= rhs;** | **\<T as [**std::ops::ShlAssign**]\<U>>::[**shl_assign**]\(&mut lhs, rhs);** | () |
| Shift right | **lhs >> rhs;** | **\<T as [**std::ops::Shr**]\<U>>::[**shr**]\(lhs, rhs);** | \<T as [**std::ops::Shr**]\<U>>::[**Output**](**std::ops::Shr::Output**) |
| Shift right in-place | **lhs >>= rhs;** | **\<T as [**std::ops::ShrAssign**]\<U>>::[**shr_assign**]\(&mut lhs, rhs);** | () |
| Subtraction | **lhs - rhs;** | **\<T as [**std::ops::Sub**]\<U>>::[**sub**]\(lhs, rhs);** | \<T as [**std::ops::Sub**]\<U>>::[**Output**](**std::ops::Sub::Output**) |
| Subtraction in-place | **lhs -= rhs;** | **\<T as [**std::ops::SubAssign**]\<U>>::[**sub_assign**]\(&mut lhs, rhs);** | () |

In this example, we implemented a simple [Complex Number] library to provide example implementations of traits of some operators.

## Usage

```
$ cargo test -q --package operator-overloading
```

## Arguments and flags

None

## Example run

```
$ cargo test -q --package operator-overloading
<...>
test result: ok. <...> passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## Known bugs

None

## Limitations

- There are also [**std::ops::Fn**], [**std::ops::FnMut**], [**std::ops::FnOnce**] traits, which are used to construct [closures].
  However, since they are not manually overloadable at the moment, they are not included in this description.

## Notes

- Destruction's desugared form is not exactly as written above because the drop in there is actually [**std::mem::drop**] function, which takes its argument by moving.
  So at the end of the [**std::mem::drop**] function, [**drop**] method of the [**std::ops::Drop**] trait will be called like the desugared form automatically.
- Descriptions of the [**std::cmp**] and [**std::ops**] modules explain further details perfectly.
  So reading them is highly recommended.
- This library is not meant for production use.
  It exists only to demonstrate how to overload operators in Rust.

## Further reading

- [**std::cmp**] description.
- [**std::ops**] description.


[//]: # (Links)

[**add**]:
  https://doc.rust-lang.org/std/ops/trait.Add.html#tymethod.add
[**add_assign**]:
  https://doc.rust-lang.org/std/ops/trait.AddAssign.html#tymethod.add_assign
[**bitand**]:
  https://doc.rust-lang.org/std/ops/trait.BitAnd.html#tymethod.bitand
[**bitand_assign**]:
  https://doc.rust-lang.org/std/ops/trait.BitAndAssign.html#tymethod.bitand_assign
[**bitor**]:
  https://doc.rust-lang.org/std/ops/trait.BitOr.html#tymethod.bitor
[**bitor_assign**]:
  https://doc.rust-lang.org/std/ops/trait.BitOrAssign.html#tymethod.bitor_assign
[**bitxor**]:
  https://doc.rust-lang.org/std/ops/trait.BitXor.html#tymethod.bitxor
[**bitxor_assign**]:
  https://doc.rust-lang.org/std/ops/trait.BitXorAssign.html#tymethod.bitxor_assign
[**bool**]:
  https://doc.rust-lang.org/std/primitive.bool.html
[**deref**]:
  https://doc.rust-lang.org/std/ops/trait.Deref.html#tymethod.deref
[**deref_mut**]:
  https://doc.rust-lang.org/std/ops/trait.DerefMut.html#tymethod.deref_mut
[**div**]:
  https://doc.rust-lang.org/std/ops/trait.Div.html#tymethod.div
[**div_assign**]:
  https://doc.rust-lang.org/std/ops/trait.DivAssign.html#tymethod.div_assign
[**drop**]:
  https://doc.rust-lang.org/std/ops/trait.Drop.html#tymethod.drop
[**eq**]:
  https://doc.rust-lang.org/std/cmp/trait.PartialEq.html#tymethod.eq
[**ge**]:
  https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html#method.ge
[**gt**]:
  https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html#method.gt
[**index**]:
  https://doc.rust-lang.org/std/ops/trait.Index.html#tymethod.index
[**index_mut**]:
  https://doc.rust-lang.org/std/ops/trait.IndexMut.html#tymethod.index_mut
[**le**]:
  https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html#method.le
[**lt**]:
  https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html#method.lt
[**mul**]:
  https://doc.rust-lang.org/std/ops/trait.Mul.html#tymethod.mul
[**mul_assign**]:
  https://doc.rust-lang.org/std/ops/trait.MulAssign.html#tymethod.mul_assign
[**ne**]:
  https://doc.rust-lang.org/std/cmp/trait.PartialEq.html#method.ne
[**neg**]:
  https://doc.rust-lang.org/std/ops/trait.Neg.html#tymethod.neg
[**not**]:
  https://doc.rust-lang.org/std/ops/trait.Not.html#tymethod.not
[**rem**]:
  https://doc.rust-lang.org/std/ops/trait.Rem.html#tymethod.rem
[**rem_assign**]:
  https://doc.rust-lang.org/std/ops/trait.RemAssign.html#tymethod.rem_assign
[**shl**]:
  https://doc.rust-lang.org/std/ops/trait.Shl.html#tymethod.shl
[**shl_assign**]:
  https://doc.rust-lang.org/std/ops/trait.ShlAssign.html#tymethod.shl_assign
[**shr**]:
  https://doc.rust-lang.org/std/ops/trait.Shr.html#tymethod.shr
[**shr_assign**]:
  https://doc.rust-lang.org/std/ops/trait.ShrAssign.html#tymethod.shr_assign
[**std::cmp**]:
  https://doc.rust-lang.org/std/cmp/index.html
[**std::cmp::PartialEq**]:
  https://doc.rust-lang.org/std/cmp/trait.PartialEq.html
[**std::cmp::PartialOrd**]:
  https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html
[**std::mem::drop**]:
  https://doc.rust-lang.org/std/mem/fn.drop.html
[**std::ops**]:
  https://doc.rust-lang.org/std/ops/index.html
[**std::ops::Add**]:
  https://doc.rust-lang.org/std/ops/trait.Add.html
[**std::ops::Add::Output**]:
  https://doc.rust-lang.org/std/ops/trait.Add.html#associatedtype.Output
[**std::ops::AddAssign**]:
  https://doc.rust-lang.org/std/ops/trait.AddAssign.html
[**std::ops::BitAnd**]:
  https://doc.rust-lang.org/std/ops/trait.BitAnd.html
[**std::ops::BitAnd::Output**]:
  https://doc.rust-lang.org/std/ops/trait.BitAnd.html#associatedtype.Output
[**std::ops::BitAndAssign**]:
  https://doc.rust-lang.org/std/ops/trait.BitAndAssign.html
[**std::ops::BitOr**]:
  https://doc.rust-lang.org/std/ops/trait.BitOr.html
[**std::ops::BitOr::Output**]:
  https://doc.rust-lang.org/std/ops/trait.BitOr.html#associatedtype.Output
[**std::ops::BitOrAssign**]:
  https://doc.rust-lang.org/std/ops/trait.BitOrAssign.html
[**std::ops::BitXor**]:
  https://doc.rust-lang.org/std/ops/trait.BitXor.html
[**std::ops::BitXor::Output**]:
  https://doc.rust-lang.org/std/ops/trait.BitXor.html#associatedtype.Output
[**std::ops::BitXorAssign**]:
  https://doc.rust-lang.org/std/ops/trait.BitXorAssign.html
[**std::ops::Deref**]:
  https://doc.rust-lang.org/std/ops/trait.Deref.html
[**std::ops::Deref::Target**]:
  https://doc.rust-lang.org/std/ops/trait.Deref.html#associatedtype.Target
[**std::ops::DerefMut**]:
  https://doc.rust-lang.org/std/ops/trait.DerefMut.html
[**std::ops::DerefMut::Target**]:
  https://doc.rust-lang.org/std/ops/trait.Deref.html#associatedtype.Target
[**std::ops::Div**]:
  https://doc.rust-lang.org/std/ops/trait.Div.html
[**std::ops::Div::Output**]:
  https://doc.rust-lang.org/std/ops/trait.Div.html#associatedtype.Output
[**std::ops::DivAssign**]:
  https://doc.rust-lang.org/std/ops/trait.DivAssign.html
[**std::ops::Drop**]:
  https://doc.rust-lang.org/std/ops/trait.Drop.html
[**std::ops::Fn**]:
  https://doc.rust-lang.org/std/ops/trait.Fn.html
[**std::ops::FnMut**]:
  https://doc.rust-lang.org/std/ops/trait.FnMut.html
[**std::ops::FnOnce**]:
  https://doc.rust-lang.org/std/ops/trait.FnOnce.html
[**std::ops::Index**]:
  https://doc.rust-lang.org/std/ops/trait.Index.html
[**std::ops::Index::Output**]:
  https://doc.rust-lang.org/std/ops/trait.Index.html#associatedtype.Output
[**std::ops::IndexMut**]:
  https://doc.rust-lang.org/std/ops/trait.IndexMut.html
[**std::ops::IndexMut::Output**]:
  https://doc.rust-lang.org/std/ops/trait.IndexMut.html#associatedtype.Output
[**std::ops::Mul**]:
  https://doc.rust-lang.org/std/ops/trait.Mul.html
[**std::ops::Mul::Output**]:
  https://doc.rust-lang.org/std/ops/trait.Mul.html#associatedtype.Output
[**std::ops::MulAssign**]:
  https://doc.rust-lang.org/std/ops/trait.MulAssign.html
[**std::ops::Neg**]:
  https://doc.rust-lang.org/std/ops/trait.Neg.html
[**std::ops::Neg::Output**]:
  https://doc.rust-lang.org/std/ops/trait.Neg.html#associatedtype.Output
[**std::ops::Not**]:
  https://doc.rust-lang.org/std/ops/trait.Not.html
[**std::ops::Not::Output**]:
  https://doc.rust-lang.org/std/ops/trait.Not.html#associatedtype.Output
[**std::ops::Rem**]:
  https://doc.rust-lang.org/std/ops/trait.Rem.html
[**std::ops::Rem::Output**]:
  https://doc.rust-lang.org/std/ops/trait.Rem.html#associatedtype.Output
[**std::ops::RemAssign**]:
  https://doc.rust-lang.org/std/ops/trait.RemAssign.html
[**std::ops::Shl**]:
  https://doc.rust-lang.org/std/ops/trait.Shl.html
[**std::ops::Shl::Output**]:
  https://doc.rust-lang.org/std/ops/trait.Shl.html#associatedtype.Output
[**std::ops::ShlAssign**]:
  https://doc.rust-lang.org/std/ops/trait.ShlAssign.html
[**std::ops::Shr**]:
  https://doc.rust-lang.org/std/ops/trait.Shr.html
[**std::ops::Shr::Output**]:
  https://doc.rust-lang.org/std/ops/trait.Shr.html#associatedtype.Output
[**std::ops::ShrAssign**]:
  https://doc.rust-lang.org/std/ops/trait.ShrAssign.html
[**std::ops::Sub**]:
  https://doc.rust-lang.org/std/ops/trait.Sub.html
[**std::ops::Sub::Output**]:
  https://doc.rust-lang.org/std/ops/trait.Sub.html#associatedtype.Output
[**std::ops::SubAssign**]:
  https://doc.rust-lang.org/std/ops/trait.SubAssign.html
[**sub**]:
  https://doc.rust-lang.org/std/ops/trait.Sub.html#tymethod.sub
[**sub_assign**]:
  https://doc.rust-lang.org/std/ops/trait.SubAssign.html#tymethod.sub_assign
[Complex Number]:
  https://en.wikipedia.org/wiki/Complex_number
[closures]:
  https://doc.rust-lang.org/1.30.0/book/2018-edition/ch13-01-closures.html
