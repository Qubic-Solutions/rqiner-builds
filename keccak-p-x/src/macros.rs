#![macro_use]

macro_rules! copy_from_state {
    (
        $lanes:ident,
        $b_ba: ident, $b_be: ident, $b_bi: ident, $b_bo: ident, $b_bu: ident,
        $b_ga: ident, $b_ge: ident, $b_gi: ident, $b_go: ident, $b_gu: ident,
        $b_ka: ident, $b_ke: ident, $b_ki: ident, $b_ko: ident, $b_ku: ident,
        $b_ma: ident, $b_me: ident, $b_mi: ident, $b_mo: ident, $b_mu: ident,
        $b_sa: ident, $b_se: ident, $b_si: ident, $b_so: ident, $b_su: ident,
        $c_a: ident, $c_e: ident, $c_i: ident, $c_o: ident, $c_u: ident,
        $d_a: ident, $d_e: ident, $d_i: ident, $d_o: ident, $d_u: ident,
        $e_ba: ident, $e_be: ident, $e_bi: ident, $e_bo: ident, $e_bu: ident,
        $e_ga: ident, $e_ge: ident, $e_gi: ident, $e_go: ident, $e_gu: ident,
        $e_ka: ident, $e_ke: ident, $e_ki: ident, $e_ko: ident, $e_ku: ident,
        $e_ma: ident, $e_me: ident, $e_mi: ident, $e_mo: ident, $e_mu: ident,
        $e_sa: ident, $e_se: ident, $e_si: ident, $e_so: ident, $e_su: ident,
    ) => {
        let mut $b_ba;
        let mut $b_be;
        let mut $b_bi;
        let mut $b_bo;
        let mut $b_bu;
        let mut $b_ga;
        let mut $b_ge;
        let mut $b_gi;
        let mut $b_go;
        let mut $b_gu;
        let mut $b_ka;
        let mut $b_ke;
        let mut $b_ki;
        let mut $b_ko;
        let mut $b_ku;
        let mut $b_ma;
        let mut $b_me;
        let mut $b_mi;
        let mut $b_mo;
        let mut $b_mu;
        let mut $b_sa;
        let mut $b_se;
        let mut $b_si;
        let mut $b_so;
        let mut $b_su;
        let mut $c_a = $lanes[0] ^ $lanes[5] ^ $lanes[10] ^ $lanes[15] ^ $lanes[20];
        let mut $c_e = $lanes[1] ^ $lanes[6] ^ $lanes[11] ^ $lanes[16] ^ $lanes[21];
        let mut $c_i = $lanes[2] ^ $lanes[7] ^ $lanes[12] ^ $lanes[17] ^ $lanes[22];
        let mut $c_o = $lanes[3] ^ $lanes[8] ^ $lanes[13] ^ $lanes[18] ^ $lanes[23];
        let mut $c_u = $lanes[4] ^ $lanes[9] ^ $lanes[14] ^ $lanes[19] ^ $lanes[24];
        let mut $d_a;
        let mut $d_e;
        let mut $d_i;
        let mut $d_o;
        let mut $d_u;
        let mut $e_ba;
        let mut $e_be;
        let mut $e_bi;
        let mut $e_bo;
        let mut $e_bu;
        let mut $e_ga;
        let mut $e_ge;
        let mut $e_gi;
        let mut $e_go;
        let mut $e_gu;
        let mut $e_ka;
        let mut $e_ke;
        let mut $e_ki;
        let mut $e_ko;
        let mut $e_ku;
        let mut $e_ma;
        let mut $e_me;
        let mut $e_mi;
        let mut $e_mo;
        let mut $e_mu;
        let mut $e_sa;
        let mut $e_se;
        let mut $e_si;
        let mut $e_so;
        let mut $e_su;
    };
}

macro_rules! double_round {
    (
        $rc_a: tt, $rc_b: tt,
        $lanes: ident,
        $b_ba: ident, $b_be: ident, $b_bi: ident, $b_bo: ident, $b_bu: ident,
        $b_ga: ident, $b_ge: ident, $b_gi: ident, $b_go: ident, $b_gu: ident,
        $b_ka: ident, $b_ke: ident, $b_ki: ident, $b_ko: ident, $b_ku: ident,
        $b_ma: ident, $b_me: ident, $b_mi: ident, $b_mo: ident, $b_mu: ident,
        $b_sa: ident, $b_se: ident, $b_si: ident, $b_so: ident, $b_su: ident,
        $c_a: ident, $c_e: ident, $c_i: ident, $c_o: ident, $c_u: ident,
        $d_a: ident, $d_e: ident, $d_i: ident, $d_o: ident, $d_u: ident,
        $e_ba: ident, $e_be: ident, $e_bi: ident, $e_bo: ident, $e_bu: ident,
        $e_ga: ident, $e_ge: ident, $e_gi: ident, $e_go: ident, $e_gu: ident,
        $e_ka: ident, $e_ke: ident, $e_ki: ident, $e_ko: ident, $e_ku: ident,
        $e_ma: ident, $e_me: ident, $e_mi: ident, $e_mo: ident, $e_mu: ident,
        $e_sa: ident, $e_se: ident, $e_si: ident, $e_so: ident, $e_su: ident,
    ) => {
        $d_a = $c_u ^ $c_e.rotate_left(1);
        $d_e = $c_a ^ $c_i.rotate_left(1);
        $d_i = $c_e ^ $c_o.rotate_left(1);
        $d_o = $c_i ^ $c_u.rotate_left(1);
        $d_u = $c_o ^ $c_a.rotate_left(1);
        $lanes[0] ^= $d_a;
        $b_ba = $lanes[0];
        $lanes[6] ^= $d_e;
        $b_be = $lanes[6].rotate_left(44);
        $lanes[12] ^= $d_i;
        $b_bi = $lanes[12].rotate_left(43);
        $lanes[18] ^= $d_o;
        $b_bo = $lanes[18].rotate_left(21);
        $lanes[24] ^= $d_u;
        $b_bu = $lanes[24].rotate_left(14);
        $e_ba = $b_ba ^ ((!$b_be) & $b_bi);
        $e_ba ^= $rc_a;
        $c_a = $e_ba;
        $e_be = $b_be ^ ((!$b_bi) & $b_bo);
        $c_e = $e_be;
        $e_bi = $b_bi ^ ((!$b_bo) & $b_bu);
        $c_i = $e_bi;
        $e_bo = $b_bo ^ ((!$b_bu) & $b_ba);
        $c_o = $e_bo;
        $e_bu = $b_bu ^ ((!$b_ba) & $b_be);
        $c_u = $e_bu;
        $lanes[3] ^= $d_o;
        $b_ga = $lanes[3].rotate_left(28);
        $lanes[9] ^= $d_u;
        $b_ge = $lanes[9].rotate_left(20);
        $lanes[10] ^= $d_a;
        $b_gi = $lanes[10].rotate_left(3);
        $lanes[16] ^= $d_e;
        $b_go = $lanes[16].rotate_left(45);
        $lanes[22] ^= $d_i;
        $b_gu = $lanes[22].rotate_left(61);
        $e_ga = $b_ga ^ ((!$b_ge) & $b_gi);
        $c_a ^= $e_ga;
        $e_ge = $b_ge ^ ((!$b_gi) & $b_go);
        $c_e ^= $e_ge;
        $e_gi = $b_gi ^ ((!$b_go) & $b_gu);
        $c_i ^= $e_gi;
        $e_go = $b_go ^ ((!$b_gu) & $b_ga);
        $c_o ^= $e_go;
        $e_gu = $b_gu ^ ((!$b_ga) & $b_ge);
        $c_u ^= $e_gu;
        $lanes[1] ^= $d_e;
        $b_ka = $lanes[1].rotate_left(1);
        $lanes[7] ^= $d_i;
        $b_ke = $lanes[7].rotate_left(6);
        $lanes[13] ^= $d_o;
        $b_ki = $lanes[13].rotate_left(25);
        $lanes[19] ^= $d_u;
        $b_ko = $lanes[19].rotate_left(8);
        $lanes[20] ^= $d_a;
        $b_ku = $lanes[20].rotate_left(18);
        $e_ka = $b_ka ^ ((!$b_ke) & $b_ki);
        $c_a ^= $e_ka;
        $e_ke = $b_ke ^ ((!$b_ki) & $b_ko);
        $c_e ^= $e_ke;
        $e_ki = $b_ki ^ ((!$b_ko) & $b_ku);
        $c_i ^= $e_ki;
        $e_ko = $b_ko ^ ((!$b_ku) & $b_ka);
        $c_o ^= $e_ko;
        $e_ku = $b_ku ^ ((!$b_ka) & $b_ke);
        $c_u ^= $e_ku;
        $lanes[4] ^= $d_u;
        $b_ma = $lanes[4].rotate_left(27);
        $lanes[5] ^= $d_a;
        $b_me = $lanes[5].rotate_left(36);
        $lanes[11] ^= $d_e;
        $b_mi = $lanes[11].rotate_left(10);
        $lanes[17] ^= $d_i;
        $b_mo = $lanes[17].rotate_left(15);
        $lanes[23] ^= $d_o;
        $b_mu = $lanes[23].rotate_left(56);
        $e_ma = $b_ma ^ ((!$b_me) & $b_mi);
        $c_a ^= $e_ma;
        $e_me = $b_me ^ ((!$b_mi) & $b_mo);
        $c_e ^= $e_me;
        $e_mi = $b_mi ^ ((!$b_mo) & $b_mu);
        $c_i ^= $e_mi;
        $e_mo = $b_mo ^ ((!$b_mu) & $b_ma);
        $c_o ^= $e_mo;
        $e_mu = $b_mu ^ ((!$b_ma) & $b_me);
        $c_u ^= $e_mu;
        $lanes[2] ^= $d_i;
        $b_sa = $lanes[2].rotate_left(62);
        $lanes[8] ^= $d_o;
        $b_se = $lanes[8].rotate_left(55);
        $lanes[14] ^= $d_u;
        $b_si = $lanes[14].rotate_left(39);
        $lanes[15] ^= $d_a;
        $b_so = $lanes[15].rotate_left(41);
        $lanes[21] ^= $d_e;
        $b_su = $lanes[21].rotate_left(2);
        $e_sa = $b_sa ^ ((!$b_se) & $b_si);
        $c_a ^= $e_sa;
        $e_se = $b_se ^ ((!$b_si) & $b_so);
        $c_e ^= $e_se;
        $e_si = $b_si ^ ((!$b_so) & $b_su);
        $c_i ^= $e_si;
        $e_so = $b_so ^ ((!$b_su) & $b_sa);
        $c_o ^= $e_so;
        $e_su = $b_su ^ ((!$b_sa) & $b_se);
        $c_u ^= $e_su;
        $d_a = $c_u ^ $c_e.rotate_left(1);
        $d_e = $c_a ^ $c_i.rotate_left(1);
        $d_i = $c_e ^ $c_o.rotate_left(1);
        $d_o = $c_i ^ $c_u.rotate_left(1);
        $d_u = $c_o ^ $c_a.rotate_left(1);
        $e_ba ^= $d_a;
        $b_ba = $e_ba;
        $e_ge ^= $d_e;
        $b_be = $e_ge.rotate_left(44);
        $e_ki ^= $d_i;
        $b_bi = $e_ki.rotate_left(43);
        $e_mo ^= $d_o;
        $b_bo = $e_mo.rotate_left(21);
        $e_su ^= $d_u;
        $b_bu = $e_su.rotate_left(14);
        $lanes[0] = $b_ba ^ ((!$b_be) & $b_bi);
        $lanes[0] ^= $rc_b;
        $c_a = $lanes[0];
        $lanes[1] = $b_be ^ ((!$b_bi) & $b_bo);
        $c_e = $lanes[1];
        $lanes[2] = $b_bi ^ ((!$b_bo) & $b_bu);
        $c_i = $lanes[2];
        $lanes[3] = $b_bo ^ ((!$b_bu) & $b_ba);
        $c_o = $lanes[3];
        $lanes[4] = $b_bu ^ ((!$b_ba) & $b_be);
        $c_u = $lanes[4];
        $e_bo ^= $d_o;
        $b_ga = $e_bo.rotate_left(28);
        $e_gu ^= $d_u;
        $b_ge = $e_gu.rotate_left(20);
        $e_ka ^= $d_a;
        $b_gi = $e_ka.rotate_left(3);
        $e_me ^= $d_e;
        $b_go = $e_me.rotate_left(45);
        $e_si ^= $d_i;
        $b_gu = $e_si.rotate_left(61);
        $lanes[5] = $b_ga ^ ((!$b_ge) & $b_gi);
        $c_a ^= $lanes[5];
        $lanes[6] = $b_ge ^ ((!$b_gi) & $b_go);
        $c_e ^= $lanes[6];
        $lanes[7] = $b_gi ^ ((!$b_go) & $b_gu);
        $c_i ^= $lanes[7];
        $lanes[8] = $b_go ^ ((!$b_gu) & $b_ga);
        $c_o ^= $lanes[8];
        $lanes[9] = $b_gu ^ ((!$b_ga) & $b_ge);
        $c_u ^= $lanes[9];
        $e_be ^= $d_e;
        $b_ka = $e_be.rotate_left(1);
        $e_gi ^= $d_i;
        $b_ke = $e_gi.rotate_left(6);
        $e_ko ^= $d_o;
        $b_ki = $e_ko.rotate_left(25);
        $e_mu ^= $d_u;
        $b_ko = $e_mu.rotate_left(8);
        $e_sa ^= $d_a;
        $b_ku = $e_sa.rotate_left(18);
        $lanes[10] = $b_ka ^ ((!$b_ke) & $b_ki);
        $c_a ^= $lanes[10];
        $lanes[11] = $b_ke ^ ((!$b_ki) & $b_ko);
        $c_e ^= $lanes[11];
        $lanes[12] = $b_ki ^ ((!$b_ko) & $b_ku);
        $c_i ^= $lanes[12];
        $lanes[13] = $b_ko ^ ((!$b_ku) & $b_ka);
        $c_o ^= $lanes[13];
        $lanes[14] = $b_ku ^ ((!$b_ka) & $b_ke);
        $c_u ^= $lanes[14];
        $e_bu ^= $d_u;
        $b_ma = $e_bu.rotate_left(27);
        $e_ga ^= $d_a;
        $b_me = $e_ga.rotate_left(36);
        $e_ke ^= $d_e;
        $b_mi = $e_ke.rotate_left(10);
        $e_mi ^= $d_i;
        $b_mo = $e_mi.rotate_left(15);
        $e_so ^= $d_o;
        $b_mu = $e_so.rotate_left(56);
        $lanes[15] = $b_ma ^ ((!$b_me) & $b_mi);
        $c_a ^= $lanes[15];
        $lanes[16] = $b_me ^ ((!$b_mi) & $b_mo);
        $c_e ^= $lanes[16];
        $lanes[17] = $b_mi ^ ((!$b_mo) & $b_mu);
        $c_i ^= $lanes[17];
        $lanes[18] = $b_mo ^ ((!$b_mu) & $b_ma);
        $c_o ^= $lanes[18];
        $lanes[19] = $b_mu ^ ((!$b_ma) & $b_me);
        $c_u ^= $lanes[19];
        $e_bi ^= $d_i;
        $b_sa = $e_bi.rotate_left(62);
        $e_go ^= $d_o;
        $b_se = $e_go.rotate_left(55);
        $e_ku ^= $d_u;
        $b_si = $e_ku.rotate_left(39);
        $e_ma ^= $d_a;
        $b_so = $e_ma.rotate_left(41);
        $e_se ^= $d_e;
        $b_su = $e_se.rotate_left(2);
        $lanes[20] = $b_sa ^ ((!$b_se) & $b_si);
        $c_a ^= $lanes[20];
        $lanes[21] = $b_se ^ ((!$b_si) & $b_so);
        $c_e ^= $lanes[21];
        $lanes[22] = $b_si ^ ((!$b_so) & $b_su);
        $c_i ^= $lanes[22];
        $lanes[23] = $b_so ^ ((!$b_su) & $b_sa);
        $c_o ^= $lanes[23];
        $lanes[24] = $b_su ^ ((!$b_sa) & $b_se);
        $c_u ^= $lanes[24];
    };
}

macro_rules! iter_rounds {
    ($lanes: ident, $( ($rc_a:expr, $rc_b:expr)),*) => {
        copy_from_state!(
            $lanes,
            b_ba, b_be, b_bi, b_bo, b_bu,
            b_ga, b_ge, b_gi, b_go, b_gu,
            b_ka, b_ke, b_ki, b_ko, b_ku,
            b_ma, b_me, b_mi, b_mo, b_mu,
            b_sa, b_se, b_si, b_so, b_su,
            c_a, c_e, c_i, c_o, c_u,
            d_a, d_e, d_i, d_o, d_u,
            e_ba, e_be, e_bi, e_bo, e_bu,
            e_ga, e_ge, e_gi, e_go, e_gu,
            e_ka, e_ke, e_ki, e_ko, e_ku,
            e_ma, e_me, e_mi, e_mo, e_mu,
            e_sa, e_se, e_si, e_so, e_su,
        );
        $(
            double_round!(
                $rc_a, $rc_b,
                $lanes,
                b_ba, b_be, b_bi, b_bo, b_bu,
                b_ga, b_ge, b_gi, b_go, b_gu,
                b_ka, b_ke, b_ki, b_ko, b_ku,
                b_ma, b_me, b_mi, b_mo, b_mu,
                b_sa, b_se, b_si, b_so, b_su,
                c_a, c_e, c_i, c_o, c_u,
                d_a, d_e, d_i,d_o, d_u,
                e_ba, e_be, e_bi, e_bo, e_bu,
                e_ga, e_ge, e_gi, e_go, e_gu,
                e_ka, e_ke, e_ki, e_ko, e_ku,
                e_ma, e_me, e_mi, e_mo, e_mu,
                e_sa, e_se, e_si, e_so, e_su,
            );
        )*
    };
}