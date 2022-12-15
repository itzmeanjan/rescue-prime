#pragma once
#include "ff.hpp"
#include <cstring>

// Rescue Permutation over prime field Z_q, q = 2^64 - 2^32 + 1
//
// Constants are taken from
// https://github.com/novifinancial/winterfell/blob/437dc08/crypto/src/hash/rescue/rp64_256/mod.rs#L252-L269
namespace rescue {

// Capacity portion of Rescue permutation state begins at index 0
constexpr size_t CAPACITY_BEGINS = 0ul;

// Capacity portion of Rescue permutation state spans over first four elements
// of the state i.e. starting at index 0 and ending at index 3.
constexpr size_t CAPACITY = 4ul;

// Rate portion of Rescue permutation state begins at index 4
constexpr size_t RATE_BEGINS = CAPACITY_BEGINS + CAPACITY;

// Rate portion of Rescue permutation state spans over last eight elements of
// the state i.e. starting at index 4 and ending at index 11.
constexpr size_t RATE = 8ul;

// Rescue permutation state is 12 elements wide where each index holds an
// element ∈ Z_q
constexpr size_t STATE_WIDTH = CAPACITY + RATE;

// Digest portion of Rescue permutation state begins at index 4
constexpr size_t DIGEST_BEGINS = RATE_BEGINS;

// Digest portion of Rescue permutation state spans over first four elements of
// the rate portion i.e. starting at index 4 and ending at index 7.
constexpr size_t DIGEST_WIDTH = 4ul;

// Rescue permutation requires application of 7 rounds to target 128 -bit
// security with 40% security margin
constexpr size_t ROUNDS = 7ul;

// S-Box power, taken from
// https://github.com/novifinancial/winterfell/blob/437dc08/crypto/src/hash/rescue/rp64_256/mod.rs#L45-L51
constexpr size_t ALPHA = 7ul;

// Inverse S-Box power, taken from
// https://github.com/novifinancial/winterfell/blob/437dc08/crypto/src/hash/rescue/rp64_256/mod.rs#L52-L53
constexpr size_t INV_ALPHA = 10540996611094048183ul;

// Precomputed Rescue MDS matrix, which is of dimension 12 x 12, taken from
// https://github.com/novifinancial/winterfell/blob/437dc08/crypto/src/hash/rescue/rp64_256/mod.rs#L372-L545
constexpr ff::ff_t MDS[STATE_WIDTH * STATE_WIDTH]{
  2108866337646019936ul,  11223275256334781131ul, 2318414738826783588ul,
  11240468238955543594ul, 8007389560317667115ul,  11080831380224887131ul,
  3922954383102346493ul,  17194066286743901609ul, 152620255842323114ul,
  7203302445933022224ul,  17781531460838764471ul, 2306881200ul,

  3368836954250922620ul,  5531382716338105518ul,  7747104620279034727ul,
  14164487169476525880ul, 4653455932372793639ul,  5504123103633670518ul,
  3376629427948045767ul,  1687083899297674997ul,  8324288417826065247ul,
  17651364087632826504ul, 15568475755679636039ul, 4656488262337620150ul,

  2560535215714666606ul,  10793518538122219186ul, 408467828146985886ul,
  13894393744319723897ul, 17856013635663093677ul, 14510101432365346218ul,
  12175743201430386993ul, 12012700097100374591ul, 976880602086740182ul,
  3187015135043748111ul,  4630899319883688283ul,  17674195666610532297ul,

  10940635879119829731ul, 9126204055164541072ul,  13441880452578323624ul,
  13828699194559433302ul, 6245685172712904082ul,  3117562785727957263ul,
  17389107632996288753ul, 3643151412418457029ul,  10484080975961167028ul,
  4066673631745731889ul,  8847974898748751041ul,  9548808324754121113ul,

  15656099696515372126ul, 309741777966979967ul,   16075523529922094036ul,
  5384192144218250710ul,  15171244241641106028ul, 6660319859038124593ul,
  6595450094003204814ul,  15330207556174961057ul, 2687301105226976975ul,
  15907414358067140389ul, 2767130804164179683ul,  8135839249549115549ul,

  14687393836444508153ul, 8122848807512458890ul,  16998154830503301252ul,
  2904046703764323264ul,  11170142989407566484ul, 5448553946207765015ul,
  9766047029091333225ul,  3852354853341479440ul,  14577128274897891003ul,
  11994931371916133447ul, 8299269445020599466ul,  2859592328380146288ul,

  4920761474064525703ul,  13379538658122003618ul, 3169184545474588182ul,
  15753261541491539618ul, 622292315133191494ul,   14052907820095169428ul,
  5159844729950547044ul,  17439978194716087321ul, 9945483003842285313ul,
  13647273880020281344ul, 14750994260825376ul,    12575187259316461486ul,

  3371852905554824605ul,  8886257005679683950ul,  15677115160380392279ul,
  13242906482047961505ul, 12149996307978507817ul, 1427861135554592284ul,
  4033726302273030373ul,  14761176804905342155ul, 11465247508084706095ul,
  12112647677590318112ul, 17343938135425110721ul, 14654483060427620352ul,

  5421794552262605237ul,  14201164512563303484ul, 5290621264363227639ul,
  1020180205893205576ul,  14311345105258400438ul, 7828111500457301560ul,
  9436759291445548340ul,  5716067521736967068ul,  15357555109169671716ul,
  4131452666376493252ul,  16785275933585465720ul, 11180136753375315897ul,

  10451661389735482801ul, 12128852772276583847ul, 10630876800354432923ul,
  6884824371838330777ul,  16413552665026570512ul, 13637837753341196082ul,
  2558124068257217718ul,  4327919242598628564ul,  4236040195908057312ul,
  2081029262044280559ul,  2047510589162918469ul,  6835491236529222042ul,

  5675273097893923172ul,  8120839782755215647ul,  9856415804450870143ul,
  1960632704307471239ul,  15279057263127523057ul, 17999325337309257121ul,
  72970456904683065ul,    8899624805082057509ul,  16980481565524365258ul,
  6412696708929498357ul,  13917768671775544479ul, 5505378218427096880ul,

  10318314766641004576ul, 17320192463105632563ul, 11540812969169097044ul,
  7270556942018024148ul,  4755326086930560682ul,  2193604418377108959ul,
  11681945506511803967ul, 8000243866012209465ul,  6746478642521594042ul,
  12096331252283646217ul, 13208137848575217268ul, 5548519654341606996ul
};

// Precomputed Rescue round constants, used during first half of the
// permutation, taken from
// https://github.com/novifinancial/winterfell/blob/437dc08/crypto/src/hash/rescue/rp64_256/mod.rs#L721-L828
constexpr ff::ff_t RC0[ROUNDS * STATE_WIDTH]{
  13917550007135091859ul, 16002276252647722320ul, 4729924423368391595ul,
  10059693067827680263ul, 9804807372516189948ul,  15666751576116384237ul,
  10150587679474953119ul, 13627942357577414247ul, 2323786301545403792ul,
  615170742765998613ul,   8870655212817778103ul,  10534167191270683080ul,

  14572151513649018290ul, 9445470642301863087ul,  6565801926598404534ul,
  12667566692985038975ul, 7193782419267459720ul,  11874811971940314298ul,
  17906868010477466257ul, 1237247437760523561ul,  6829882458376718831ul,
  2140011966759485221ul,  1624379354686052121ul,  50954653459374206ul,

  16288075653722020941ul, 13294924199301620952ul, 13370596140726871456ul,
  611533288599636281ul,   12865221627554828747ul, 12269498015480242943ul,
  8230863118714645896ul,  13466591048726906480ul, 10176988631229240256ul,
  14951460136371189405ul, 5882405912332577353ul,  18125144098115032453ul,

  6076976409066920174ul,  7466617867456719866ul,  5509452692963105675ul,
  14692460717212261752ul, 12980373618703329746ul, 1361187191725412610ul,
  6093955025012408881ul,  5110883082899748359ul,  8578179704817414083ul,
  9311749071195681469ul,  16965242536774914613ul, 5747454353875601040ul,

  13684212076160345083ul, 19445754899749561ul,    16618768069125744845ul,
  278225951958825090ul,   4997246680116830377ul,  782614868534172852ul,
  16423767594935000044ul, 9990984633405879434ul,  16757120847103156641ul,
  2103861168279461168ul,  16018697163142305052ul, 6479823382130993799ul,

  13957683526597936825ul, 9702819874074407511ul,  18357323897135139931ul,
  3029452444431245019ul,  1809322684009991117ul,  12459356450895788575ul,
  11985094908667810946ul, 12868806590346066108ul, 7872185587893926881ul,
  10694372443883124306ul, 8644995046789277522ul,  1422920069067375692ul,

  17619517835351328008ul, 6173683530634627901ul,  15061027706054897896ul,
  4503753322633415655ul,  11538516425871008333ul, 12777459872202073891ul,
  17842814708228807409ul, 13441695826912633916ul, 5950710620243434509ul,
  17040450522225825296ul, 8787650312632423701ul,  7431110942091427450ul,
};

// Precomputed Rescue round constants, used during last half of the
// permutation, taken from
// https://github.com/novifinancial/winterfell/blob/437dc08/crypto/src/hash/rescue/rp64_256/mod.rs#L830-L929
constexpr ff::ff_t RC1[ROUNDS * STATE_WIDTH]{
  7989257206380839449ul,  8639509123020237648ul,  6488561830509603695ul,
  5519169995467998761ul,  2972173318556248829ul,  14899875358187389787ul,
  14160104549881494022ul, 5969738169680657501ul,  5116050734813646528ul,
  12120002089437618419ul, 17404470791907152876ul, 2718166276419445724ul,

  2485377440770793394ul,  14358936485713564605ul, 3327012975585973824ul,
  6001912612374303716ul,  17419159457659073951ul, 11810720562576658327ul,
  14802512641816370470ul, 751963320628219432ul,   9410455736958787393ul,
  16405548341306967018ul, 6867376949398252373ul,  13982182448213113532ul,

  10436926105997283389ul, 13237521312283579132ul, 668335841375552722ul,
  2385521647573044240ul,  3874694023045931809ul,  12952434030222726182ul,
  1972984540857058687ul,  14000313505684510403ul, 976377933822676506ul,
  8407002393718726702ul,  338785660775650958ul,   4208211193539481671ul,

  2284392243703840734ul,  4500504737691218932ul,  3976085877224857941ul,
  2603294837319327956ul,  5760259105023371034ul,  2911579958858769248ul,
  18415938932239013434ul, 7063156700464743997ul,  16626114991069403630ul,
  163485390956217960ul,   11596043559919659130ul, 2976841507452846995ul,

  15090073748392700862ul, 3496786927732034743ul,  8646735362535504000ul,
  2460088694130347125ul,  3944675034557577794ul,  14781700518249159275ul,
  2857749437648203959ul,  8505429584078195973ul,  18008150643764164736ul,
  720176627102578275ul,   7038653538629322181ul,  8849746187975356582ul,

  17427790390280348710ul, 1159544160012040055ul,  17946663256456930598ul,
  6338793524502945410ul,  17715539080731926288ul, 4208940652334891422ul,
  12386490721239135719ul, 10010817080957769535ul, 5566101162185411405ul,
  12520146553271266365ul, 4972547404153988943ul,  5597076522138709717ul,

  18338863478027005376ul, 115128380230345639ul,   4427489889653730058ul,
  10890727269603281956ul, 7094492770210294530ul,  7345573238864544283ul,
  6834103517673002336ul,  14002814950696095900ul, 15939230865809555943ul,
  12717309295554119359ul, 4130723396860574906ul,  7706153020203677238ul,
};

// Raise an element ∈ Z_q to its 7-th power, by using less multiplications, than
// one would do if done using standard exponentiation routine.
//
// Adapted from
// https://github.com/novifinancial/winterfell/blob/437dc08/math/src/field/f64/mod.rs#L74-L82
static inline ff::ff_t
exp7(const ff::ff_t v)
{
  const ff::ff_t v2 = v * v;
  const ff::ff_t v4 = v2 * v2;
  const ff::ff_t v6 = v2 * v4;
  const ff::ff_t v7 = v * v6;

  return v7;
}

// Taken from
// https://github.com/novifinancial/winterfell/blob/437dc08/crypto/src/hash/rescue/mod.rs#L17-L25,
// to perform cheaper exponentiation
template<const size_t m>
static inline void
exp_acc(const ff::ff_t* const base,
        const ff::ff_t* const tail,
        ff::ff_t* const __restrict res)
{
  std::memcpy(res, base, sizeof(ff::ff_t) * STATE_WIDTH);

  for (size_t i = 0; i < m; i++) {

#if defined __GNUC__
#pragma GCC unroll 12
#elif defined __clang__
#pragma unroll 12
#endif
    for (size_t j = 0; j < STATE_WIDTH; j++) {
      res[j] = res[j] * res[j];
    }
  }

#if defined __GNUC__
#pragma GCC unroll 12
#elif defined __clang__
#pragma unroll 12
#endif
  for (size_t i = 0; i < STATE_WIDTH; i++) {
    res[i] = res[i] * tail[i];
  }
}

// Applies substitution box on Rescue permutation state, by raising each element
// to its 7-th power.
static inline void
apply_sbox(ff::ff_t* const state)
{
#if defined __GNUC__
#pragma GCC unroll 12
#elif defined __clang__
#pragma unroll 12
#endif
  for (size_t i = 0; i < STATE_WIDTH; i++) {
    state[i] = exp7(state[i]);
  }
}

// Applies inverse substitution box on Rescue permutation state, by raising each
// element to its 10540996611094048183-th power, with lesser many
// multiplications.
//
// Adapted from
// https://github.com/novifinancial/winterfell/blob/437dc08/crypto/src/hash/rescue/rp64_256/mod.rs#L335-L369
static inline void
apply_inv_sbox(ff::ff_t* const state)
{
  ff::ff_t t1[STATE_WIDTH];

#if defined __GNUC__
#pragma GCC unroll 12
#elif defined __clang__
#pragma unroll 12
#endif
  for (size_t i = 0; i < STATE_WIDTH; i++) {
    t1[i] = state[i] * state[i];
  }

  ff::ff_t t2[STATE_WIDTH];

#if defined __GNUC__
#pragma GCC unroll 12
#elif defined __clang__
#pragma unroll 12
#endif
  for (size_t i = 0; i < STATE_WIDTH; i++) {
    t2[i] = t1[i] * t1[i];
  }

  ff::ff_t t3[STATE_WIDTH];
  exp_acc<3>(t2, t2, t3);

  ff::ff_t t4[STATE_WIDTH];
  exp_acc<6>(t3, t3, t4);

  ff::ff_t t5[STATE_WIDTH];
  exp_acc<12>(t4, t4, t5);

  ff::ff_t t6[STATE_WIDTH];
  exp_acc<6>(t5, t3, t6);

  ff::ff_t t7[STATE_WIDTH];
  exp_acc<31>(t6, t6, t7);

#if defined __GNUC__
#pragma GCC unroll 12
#elif defined __clang__
#pragma unroll 12
#endif
  for (size_t i = 0; i < STATE_WIDTH; i++) {
    const auto a0 = t7[i] * t7[i];
    const auto a1 = a0 * t6[i];
    const auto a2 = a1 * a1;
    const auto a3 = a2 * a2;

    const auto b0 = t1[i] * t2[i];
    const auto b1 = b0 * state[i];

    state[i] = a3 * b1;
  }
}

// Adds round constants to Rescue permutation state.
//
// Note this routine is used during the first half of Rescue permutation.
static inline void
add_rc0(ff::ff_t* const state, const size_t ridx)
{
  const size_t rc_off = ridx * STATE_WIDTH;

#if defined __GNUC__
#pragma GCC unroll 12
#elif defined __clang__
#pragma unroll 12
#endif
  for (size_t i = 0; i < STATE_WIDTH; i++) {
    state[i] = state[i] + RC0[rc_off + i];
  }
}

// Adds round constants to Rescue permutation state.
//
// Note this routine is used during the last half of Rescue permutation.
static inline void
add_rc1(ff::ff_t* const state, const size_t ridx)
{
  const size_t rc_off = ridx * STATE_WIDTH;

#if defined __GNUC__
#pragma GCC unroll 12
#elif defined __clang__
#pragma unroll 12
#endif
  for (size_t i = 0; i < STATE_WIDTH; i++) {
    state[i] = state[i] + RC1[rc_off + i];
  }
}

// Multiplies Rescue permutation state by MDS matrix
static inline void
apply_mds(ff::ff_t* const state)
{
  ff::ff_t tmp[STATE_WIDTH]{};

  for (size_t i = 0; i < STATE_WIDTH; i++) {
    const size_t off = i * STATE_WIDTH;

#if defined __GNUC__
#pragma GCC unroll 12
#elif defined __clang__
#pragma unroll 12
#endif
    for (size_t j = 0; j < STATE_WIDTH; j++) {
      tmp[i] = tmp[i] + state[j] * MDS[off + j];
    }
  }

  std::memcpy(state, tmp, sizeof(tmp));
}

// Apply single Rescue permutation round
static inline void
apply_round(ff::ff_t* const state, const size_t ridx)
{
  // first half
  apply_sbox(state);
  apply_mds(state);
  add_rc0(state, ridx);

  // second half
  apply_inv_sbox(state);
  apply_mds(state);
  add_rc1(state, ridx);
}

// Rescue Permutation of 7 rounds
static inline void
permute(ff::ff_t* const state)
{
  for (size_t i = 0; i < ROUNDS; i++) {
    apply_round(state, i);
  }
}

}
