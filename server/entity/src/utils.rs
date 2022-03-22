use sea_orm::{ColumnTrait, sea_query::SimpleExpr};

pub trait ColumnFinder<T: ColumnTrait> {
  fn find_col_by_name(col_name: &str) -> Option<T>;
}


// $eq (=, equal)
// $ne (!=, not equal)
// $gt (>, greater than)
// $lt (<, lower that)
// $gte (>=, greater than or equal)
// $lte (<=, lower than or equal)
// $starts (LIKE val%, starts with)
// $ends (LIKE %val, ends with)
// $cont (LIKE %val%, contains)
// $excl (NOT LIKE %val%, not contains)
// $in (IN, in range, accepts multiple values)
// $notin (NOT IN, not in range, accepts multiple values)
// $isnull (IS NULL, is NULL, doesn't accept value)
// $notnull (IS NOT NULL, not NULL, doesn't accept value)
// $between (BETWEEN, between, accepts two values)
// $eqL (LOWER(field) =, equal)
// $neL (LOWER(field) !=, not equal)
// $startsL (LIKE|ILIKE val%)
// $endsL (LIKE|ILIKE %val, ends with)
// $contL (LIKE|ILIKE %val%, contains)
// $exclL (NOT LIKE|ILIKE %val%, not contains)
// $inL (LOWER(field) IN, in range, accepts multiple values)
// $notinL (LOWER(field) NOT IN, not in range, accepts multiple values)

pub fn get_expr<T: ColumnTrait>(col: T, filter_condition: &str, value: &str) -> Option<SimpleExpr> {
  match filter_condition {
    "$cont" => {
      Some(col.contains(value))
    },
    _ => None,
  }
}
