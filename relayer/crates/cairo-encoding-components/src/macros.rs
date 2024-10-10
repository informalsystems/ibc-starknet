#[macro_export]
macro_rules! Sum {
  ( $(,)? ) => {
    $crate::types::either::Void
  };
  ( $e:ty ) => {
    $crate::types::either::Either<
      $e,
      $crate::types::either::Void
    >
  };
  ( $e:ty, $($tail:tt)* ) => {
    $crate::types::either::Either<
      $e,
      $crate::Sum!( $($tail)* )
    >
  };
}
