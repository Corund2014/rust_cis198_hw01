/// #[derive(...)] statements define certain properties on the enum for you for
/// free (printing, equality testing, the ability to copy values). More on this
/// when we cover Enums in detail.

/// You can use any of the variants of the `Peg` enum by writing `Peg::B`, etc.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Peg {
    A,
    B,
    C,
}

/// A move between two pegs: (source, destination).
pub type Move = (Peg, Peg);

/// Solves for the sequence of moves required to move all discs from `src` to
/// `dst`.
pub fn hanoi(num_discs: u32, src: Peg, aux: Peg, dst: Peg) -> Vec<Move> {
    let mut moves:Vec<Move>=Vec::new();
    let mut src_peg:Vec<u32>=fill_src(num_discs);
    let mut aux_peg:Vec<u32>=Vec::new();
    let mut dst_peg:Vec<u32>=Vec::new();
    let mut possible_moves:Vec<Move>=Vec::new();
    let target_result=src_peg.clone();
    while dst_peg!=target_result
    {
        generate_possible_moves(&mut possible_moves,&src_peg,&aux_peg,&dst_peg,&src,&aux,&dst);
        apply_constraints(&mut possible_moves,&src_peg,&aux_peg,&dst_peg,&src,&aux,&dst);//should remove all moves except 1 possible move
        moves.push(possible_moves.pop());
    }
    moves
}

fn fill_src(num_discs:u32)->Vec<u32>
{
    let mut src_peg:Vec<u32>=Vec::new();
    let iter=(1..num_discs+1).rev();
    for i in iter
    {
        src_peg.push(i);
    }

    src_peg
}

fn generate_possible_moves(possible_moves:&mut Vec<Move>,src_vec:&Vec<u32>,aux_vec:&Vec<u32>,dst_vec:&Vec<u32>,src: &Peg, aux: &Peg, dst: &Peg)->()
{
    assert!(possible_moves.is_empty());//should be empty at beginning
    if !src_vec.is_empty()
    {
        possible_moves.push((*src,*aux));
        possible_moves.push((*src,*dst));
    }
    if !aux_vec.is_empty()
    {
        possible_moves.push((*aux,*src));
        possible_moves.push((*aux,*dst));
    }
    if !dst_vec.is_empty()
    {
        possible_moves.push((*src,*aux));
        possible_moves.push((*src,*dst));
    }
    assert!(!possible_moves.is_empty());//no possible moves check
}

    ///apply 4 constraints:
    ///1. No odd disk may be placed directly on an odd disk.
    ///2. No even disk may be placed directly on an even disk.
    ///3. There will be sometimes two possible pegs: one will have disks, and the other will be empty. Place the disk in the non-empty peg.
    ///4. Never move a disk twice in succession.

fn generate_possible_moves(possible_moves:&mut Vec<Move>,src_vec:&Vec<u32>,aux_vec:&Vec<u32>,dst_vec:&Vec<u32>,src: &Peg, aux: &Peg, dst: &Peg)->()
{
    
}