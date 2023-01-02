/// copied and adapted from https://cloudmaker.dev/macros-for-filtering-and-pagination/
#[macro_export]
macro_rules! sort_by {
    ($query:expr, $sort_by:expr, $(($param:expr, $column:expr)),*) => {{
        if let Some(sort_by) = $sort_by {
              for sort_property in sort_by.iter() {
                $query = match sort_property.property.as_ref() {
                   $(
                       $param if sort_property.direction.to_string().eq("Desc") => $query.then_order_by($column.desc()),
                       $param => $query.then_order_by($column.asc()),
                   )*
                   _ => $query,
               }
              }
        }

        $query
    }};
}
