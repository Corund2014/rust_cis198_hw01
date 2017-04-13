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
    let mut src_vec:Vec<u32>=fill_src(num_discs);
    let mut aux_vec:Vec<u32>=Vec::new();
    let mut dst_vec:Vec<u32>=Vec::new();
    let is_even_pegs=if num_discs%2==0 {true} else {false};
    let move1:Move=if is_even_pegs {(src,aux)}else{(src,dst)};
    let move2:Move=if is_even_pegs {(src,dst)}else{(src,aux)};
    let move3:Move=(aux,dst);
    let move1swap=(move1.1,move1.0);
    let move2swap=(move2.1,move2.0);
    let move3swap=(move3.1,move3.0);
    let num=num_discs as usize;
    
    loop {
        
        if is_legal_move(&move1,&src,&aux,&dst,&src_vec,&aux_vec,&dst_vec)
        {
            moves.push(move1.clone());
        }
        else
        {
            moves.push(move1swap.clone());
        }

        if dst_vec.len()== num {break;}
        //move2
        if is_legal_move(&move2,&src,&aux,&dst,&src_vec,&aux_vec,&dst_vec)
        {
            moves.push(move2.clone());
        }
        else
        {
            moves.push(move2swap.clone());
        }        
        if dst_vec.len()==num {break;}
        //move3
        if is_legal_move(&move3,&src,&aux,&dst,&src_vec,&aux_vec,&dst_vec)
        {
            moves.push(move3.clone());
        }
        else
        {
            moves.push(move3swap.clone());
        }
        if dst_vec.len()==num {break;}
    }


    moves

}



fn is_legal_move(next_move:&Move,src: &Peg, aux: &Peg, dst: &Peg,src_vec:&Vec<u32>,aux_vec:&Vec<u32>,dst_vec:&Vec<u32>)->bool
{
    let &(start_peg,end_peg)=next_move;
    let start_vec=match start_peg {
       src => src_vec,
       aux => aux_vec,
       dst => dst_vec,
    };
    let end_vec=match end_peg {
       src => src_vec,
       aux => aux_vec,
       dst => dst_vec,
    };
    if start_vec.is_empty(){false}
    else if end_vec.is_empty() {true}
    else if start_vec.last().unwrap()<end_vec.last().unwrap(){true}
    else {false}
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
