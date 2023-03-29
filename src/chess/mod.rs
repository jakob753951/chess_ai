enum Color {
	White,
	Black
}

struct CastlingState {
	black_queenside: bool,
	black_kingside: bool,
	white_queenside: bool,
	white_kingside: bool,
}

enum PieceType {
	Pawn,
	Knight,
	Bishop,
	Rook,
	Queen,
	King,
}

struct Piece {
	piece_type: PieceType,
	color: Color,
}

struct Board {
	squares: [Option<Piece>; 8*8],
}

struct State {
	board: Board,
	turn: Color,
	castling_state: CastlingState,
	en_passant_target: Option<u8>,
	halfmove_clock: u32,
	full_moves: u32,
}