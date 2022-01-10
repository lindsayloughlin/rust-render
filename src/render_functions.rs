

    pub fn interpolate(i0 : i32, d0: i32, i1: i32, d1 : i32 ) -> Vec<i32> {

        if i0 == i1{
            return vec![d0]
        }
        let mut valOut = Vec::new();
        let a = (d1 - d0) as f32 / (i1 - i0) as f32;
        let mut d = d0 as f32;
        let mut io = i0;
        while io <= i1 {        
            valOut.push(d as i32);
            d += a;
            io = io + 1;
        }
        return valOut;

    }