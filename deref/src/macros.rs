/// Macro to implement the Deref trait, supporting both regular types and generic types
///
/// # Parameters
/// - `$ty`: The implementing type
/// - `$target`: The target type
/// - `$field`: Field access path, supports direct field names or index access
///
/// # Examples
/// ```rust
/// use deref::{deref};
///
/// struct MyType {
///     field: TargetType,
/// }
///
/// struct TargetType;
///
/// // Implement Deref for a regular type
/// deref!(MyType, TargetType, field);
/// ```
///
/// ```rust
/// use deref::deref;
///
/// struct SrVec<T> {
///     vec: Vec<T>,
/// }
///
/// // Implement Deref for a single generic type
/// deref!(<T>, SrVec<T>, Vec<T>, vec);
/// ```
///
/// ```rust
/// use deref::deref;
///
/// struct MyType<'a> {
///     field: &'a usize,
/// }
///
/// // Implement Deref for a regular type
/// deref!(<'a>, MyType<'a>, &'a usize, field);
/// ```
#[macro_export]
macro_rules! deref {
    (
        $(<
            $( $($lt:lifetime),+ )?
            $( , )?
            $( $($param:ident $(: $bound:tt)?),+ )?
        >,)?
        $ty:ident
        $(<
            $( $($lt2:lifetime),+ )?
            $( , )?
            $( $($param2:ident),+ )?
        >)?,
        $target:ty,
        $field:tt
    ) => {
        impl
        $(<
            $( $($lt),+, )?
            $( $($param $(: $bound)?),+ )?
        >)?
        std::ops::Deref for $ty
        $(<
            $( $($lt2),+, )?
            $( $($param2),+ )?
        >)?
        {
            type Target = $target;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.$field
            }
        }
    };
}

/// Macro to implement both Deref and DerefMut traits, supporting both regular types and generic types
///
/// Note: This macro automatically implements both Deref and DerefMut traits.
/// You don't need to separately use deref! when using deref_mut!.
///
/// # Parameters
/// - `$ty`: The implementing type
/// - `$target`: The target type
/// - `$field`: Field access path, supports direct field names or index access
///
/// # Examples
/// ```rust
/// use deref::{deref_mut};
///
/// struct MyType {
///     field: TargetType,
/// }
///
/// struct TargetType;
///
/// // Implement Deref and DerefMut for a regular type
/// // Note: This automatically implements both Deref and DerefMut
/// deref_mut!(MyType, TargetType, field);
/// ```
///
/// ```rust
/// use deref::{deref_mut};
///
/// struct SrVec<T> {
///     vec: Vec<T>,
/// }
///
/// // Implement Deref and DerefMut for a single generic type
/// // Note: This automatically implements both Deref and DerefMut
/// deref_mut!(<T>, SrVec<T>, Vec<T>, vec);
/// ```
///
/// ```rust
/// use deref::{deref_mut};
///
/// struct MyType<'a> {
///     field: &'a mut usize,
/// }
///
/// // Implement Deref and DerefMut for a regular type
/// // Note: This automatically implements both Deref and DerefMut
/// deref_mut!(<'a>, MyType<'a>, &'a mut usize, field);
/// ```
#[macro_export]
macro_rules! deref_mut {
    (
        $(<
            $( $($lt:lifetime),+ )?
            $( , )?
            $( $($param:ident $(: $bound:tt)?),+ )?
        >,)?
        $ty:ident
        $(<
            $( $($lt2:lifetime),+ )?
            $( , )?
            $( $($param2:ident),+ )?
        >)?,
        $target:ty,
        $field:tt
    ) => {
        $crate::deref!(
            $(<
                $( $($lt),+, )?
                $( $($param $(: $bound)?),+ )?
            >,)?
            $ty
            $(<
                $( $($lt2),+, )?
                $( $($param2),+ )?
            >)?,
            $target,
            $field
        );

        impl
        $(<
            $( $($lt),+, )?
            $( $($param $(: $bound)?),+ )?
        >)?
        std::ops::DerefMut for $ty
        $(<
            $( $($lt2),+, )?
            $( $($param2),+ )?
        >)?
        {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.$field
            }
        }
    };
}
