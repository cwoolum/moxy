use super::*;

pub(super) fn expr(cursor: Cursor<'_>, _context: ExprContext) -> bool {
    let cursor = Attributes::skip(cursor).unwrap_or(cursor);
    !cursor.is_empty() && !<Token![;]>::peek(cursor)
}

/// Whether `cursor` starts an expression with a block, which ends a match arm
/// body unless it is followed by `.` or `?`.
pub(super) fn block_like(cursor: Cursor<'_>) -> bool {
    let cursor = Label::skip(cursor).unwrap_or(cursor);

    cursor.is_delimited(Delim::Brace)
        || <Token![if]>::peek(cursor)
        || <Token![match]>::peek(cursor)
        || <Token![while]>::peek(cursor)
        || <Token![loop]>::peek(cursor)
        || (<Token![for]>::peek(cursor) && !<Token![<]>::peek(cursor.offset(1)))
        || ((<Token![unsafe]>::peek(cursor) || <Token![const]>::peek(cursor) || <Token![try]>::peek(cursor))
            && cursor.offset(1).is_delimited(Delim::Brace))
}
