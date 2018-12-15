use darkruby_chess::board;

#[test]
fn create_new_board() {
    let board = board::Board::new();
    assert!(board.fields.len() == board::LENGTH);
}

#[test]
fn board_prints_correctly() {
    let board = board::Board::new();
    assert_eq!(
        board.to_string(),
        "[ ][ ][ ][ ][ ][ ][ ][ ]
[ ][ ][ ][ ][ ][ ][ ][ ]
[ ][ ][ ][ ][ ][ ][ ][ ]
[ ][ ][ ][ ][ ][ ][ ][ ]
[ ][ ][ ][ ][ ][ ][ ][ ]
[ ][ ][ ][ ][ ][ ][ ][ ]
[ ][ ][ ][ ][ ][ ][ ][ ]
[ ][ ][ ][ ][ ][ ][ ][ ]
"
    );
}
