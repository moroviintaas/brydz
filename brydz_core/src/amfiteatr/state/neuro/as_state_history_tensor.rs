use amfiteatr_rl::tch::Tensor;
use karty::cards::DECK_SIZE;
use crate::meta::TOTAL_TRICKS;

// DECK_SIZE * 4 - initial card assumptions
//
/*
53 numbers in row

0: [bid_value] + [0/1 bid_trump] + [trump color] +
1: prediction of own hand | [1.0]
2: prediction of left hand player | [1.]
3: prediction of partner | [0]
4: prediction of right hand player |  [0]
5: actual card vector |  [0] +
6. [hand of dummy] (0 if absent/ 1 if value) |  [0/1]
//history
// trick 1
7. own card || [0/1]
8. left card || [0/1]
9. partner card || [0/1]
10. right hand card || [0/1]
[11 - 14]

*/
pub trait BuildStateHistoryTensor {
    fn contract_params(&self) -> [f32; DECK_SIZE+1];
    fn prediction(&self, relative_side: u8) -> [f32; DECK_SIZE+1];
    fn actual_cards(&self) -> [f32; DECK_SIZE+1];
    fn actual_dummy_cards(&self) -> [f32; DECK_SIZE+1];
    fn trick_cards(&self, trick_number: usize, relative_side: u8) -> [f32; DECK_SIZE+1];

    fn state_history_array(&self) -> [[f32; DECK_SIZE+1]; 7 + (4* TOTAL_TRICKS as usize)]{
        /*[
            self.contract_params(),
            self.prediction(0),
            self.prediction(1),
            self.prediction(2),
            self.prediction(3),

        ]*/
        let mut result = [[0.0; DECK_SIZE+1]; 7 + (4 * TOTAL_TRICKS as usize)];
        result[0] = self.contract_params();
        result[1] = self.prediction(0);
        result[2] = self.prediction(1);
        result[3] = self.prediction(2);
        result[4] = self.prediction(3);
        result[5] = self.actual_cards();
        result[6] = self.actual_dummy_cards();

        for trick in 0..TOTAL_TRICKS as usize{
            for s in 0..4{
                result[7+(trick* 4) + s as usize] = self.trick_cards(trick, s)
            }

        }



        result
    }

    fn state_history_tensor(&self) -> Tensor{
        Tensor::from_slice2(&self.state_history_array())
    }
    
}