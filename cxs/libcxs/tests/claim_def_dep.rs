#[cfg(test)]
mod tests {

    extern crate rusqlcipher;
    extern crate cxs;

    use self::cxs::utils::constants::{DEMO_ISSUER_PW_SEED, DEMO_AGENT_PW_SEED};
    use self::cxs::utils::signus::SignusUtils;
    use self::cxs::schema::{create_new_schema, get_sequence_num};
    use self::cxs::claim_def::create_new_claimdef;
    use self::cxs::utils::wallet;
    use self::cxs::utils::libindy::pool;

    static ENTRIES: &[[&str;3];32] = &[
        // Claim 1
        [
            "claim_definition_private::DunkM3x1y7S4ECgSL4Wkru:38",
            r#"{"secret_key":{"p":"144603162859454741168280915394909984001473642797274871180167908054497925812883981026345696503026062157878453372498485668227410562991668541345664144016049467167589411806807446732551729433505334666344135888301238316875426920088167861582189008197206702038327421351513797178773976372517828456163557215848081649501","q":"178594640881807982138486174007905085239675354219677290231202333915190751880998679358208944786547641050169273472545108431492696617713473139143668146797109824302776148000266200989359851929141615573706441864676143443456423162088061223638627189864188553699959367007074304863696099980420822529144052185249274532301"},"secret_key_revocation":null}"#,
            "2017-12-21 23:38:36"
        ],
        [
            "claim_definition::DunkM3x1y7S4ECgSL4Wkru:38",
            r#"{"ref":38,"origin":"DunkM3x1y7S4ECgSL4Wkru","signature_type":"CL","data":{"primary":{"n":"103301399765031653388723932830425639990133647845328790367497912831201491782986862136752047091274071557342752048195139812678897499728848538593238462626952717203106055286552322348411040604849141518688316459374857016140112702277496104429905735359752132897447796611106520196254724032656326955218787473249996578339875987800462991093657571161117697220348973299565170623584309401023341895388326223839510702001399833320409968051124992137598742557007123005894256688609312852594944294314563016966191879904702210196308038722202327375646862741196597722513600288076416139028262777375689457058320480830896515726645090531884252490809","s":"71963818803445570762785234241025460396976914523105712265199767176744758423400518973776860054827645328602134581240099655804497266493432098214868915551765101065086916086075773231307143246327556957174638136940955483857072489317763179787584947162888748386082767320126376315063435335520638318522932817590988977621608788417369976272634459897876619080033327883242695090055188082879778020909546833195052905362161289389231218410910531238405554365175922377296165843475630431091288038902482967167208089823547428993254810893185368467774901080963548211887324521366836011583014789974015761568535553796162735131632403563250905922435","rms":"36525713365052001310015463292289931868251129420307892545541765456927622695269459907664814766493692085327042783489459643208379475507058560951159596869932156689761732978202935627648042485700225508531818808501424080993136833867113381593527056924173012693710126008067442311023394099531206814741413376571808388978012070819384339181886940898275317940255517270836115282818064162642270571896239827654571683568687531183788530476407313461540457312408680092075647396187358840487261261578134553290641968623950561939872365758979256259525527860552516996350182981564300786318209724126564171702501792464661269298867156745647486278273","r":{"state":"21527270046586893682678402194055125703899637582855184581542449032300207314120432565147241994765626600838004072821180674935289604568152021552030097462938870352936060932010618567290036648644477282176213650374506141357844930394059583102668682138213715782458811842051432816422966190210582045859336619871992206672405714227526414630723230186105046139165411979998060681303300690452087934762569791632494461626000442694597537402686887493180455930262116802485462255375795274641744526003032842936219495494323433310836578095748672724651530511097727906976522929898685072620818966923173029894537737083955150967653212618208417742581","address1":"16343516096023879105738214840479701424177020452982862707306119014816382760869258260240095794970383271718781640777291145682420555904015063730528846144009721575020113248624808666487864073671671484541464071206970098127129160378060046180730182044929538594042169129331624088900059945091791596007313469377241689447849800467160445518154366467380122288634813741805492395079119635017292319751445682415222938061577836564925855768883999844553608143368218846319807919868404214726579746325721241933309853014298710054668126461111069233993133627782637169363895370201995156493571811753978492024968189747611133118932978104836796347548","zip":"41503357327686341553781917386401757674545115383558674173902905114461665963591074506438288214208444196629001860604487799839837258227167014240772612302180000519563568185120335598716759756348201056212165962823561566257503983343205037024883673241184885066587566160360740404373697982478988472701000157184081853678830181992476937749791732218008627194684412782330177769023180065351303103237597321256397876996589389029186913957860513235210801201428354925360105279856457289256855761694159436266299264141678768922082900920405921641614596229615213324048568483543511095820939050356399481497594976519448610326398080755073483958436","address2":"69262952205811440083954449769847644254883270169679792496068734605909538605238081223257640720964023450244651587598478840345698083387863978112765195399358226539834159644742487669914118772066414675931890973018059258543459850637353981144338684327222247460895168940279191712028157108242498320539851739099007443938573602250297808356498091982823626973006330401152075367039628717554530121296819479017982070681611775227652299998143731213378921138939051990346151269738747518605826927814051870556714663371420183140198566210820896285121312216075081928727192276894051057164186090322408125473210980054580240084799720615434823089947","city":"46131483971565146665653271011230782939681996501753456352436227633416291797924458037224617718766049555351752572063574597694972271709497217481344041919605509368941348268953613324880441813238969479677959311235439785312296275696398328853173699617896604320609812162395874527131318448013954023701414511290729464577576905162300552885120519524451189106096806820067519414676643050392929767433928526284089934262910025824636645169903475314982335809139019891750762717835221711610754581854139065220447667884483893616644092236718044642467383512820625593289751735338650751879128475794830391552867596798782814773093762283905188049960"},"rctxt":"4069400837283731061332831148308969604141143863956046320691600834762839295392948195705630544655802148386183753146933714588521148419467671354342032078538929389790389488916096598869230138644674038787981284092170941247990877011567487513252237307530106092099953479415728378469950751968066187432725915464641241557167921150591577246200489453110649521054869390269597130067104773378404735791860928144024730830691424670779540797189172684431407501104997575974030558754991837963262278149759497712964910204044622450610011148493879726861904507276056409424289089851876717354564497471579433426133515012080798417139690767425810663027","z":"42502126928320350002697582852547551252413197005137917268135311849103783021493625881283894146509298453496250856527374007672298369838184046139313809384127190344207360853104259651530530277858646293611920312304431331242925976034916478472903012032416750012962892960152004725020318636976158650054695804196932635502031476132650734979916035927241520289468123264593456034398785071639620748363763217384007867575710595233298006535023896061693400923788700228239819963296055224049470336710021362480894535639920645657908169377469278880747956463529061994392029774112008052663984287890746612741400046008758042520114287014114315983079"},"revocation":null}}"#,
            "2018-01-19 21:26:14"
        ],
        // Claim 2
        [
            "claim_definition_private::DunkM3x1y7S4ECgSL4Wkru:72",
            r#"{"secret_key":{"p":"137069362949012374641861714029301300356124405716389088406114200280863247914263819762682985741374667091282505475442591503287481526862058928795374585316033552913818871524134590979712657541779217820158389136597765889703263047868555494574832377716327876949156963289276195775152265304461052526774051145072175097529","q":"150890836229631309786101171905531037096241252779591878946395305022499186814920536260337763118142414403875249760749465372890648269746244803896198155916305718972354009665462012604931514092302151898187521264065582071564697002696814296664038944377321206377010867604461992750999111771904182924159882190313771854071"},"secret_key_revocation":null}"#,
            "2018-01-22 17:09:52"
        ],
        [
            "claim_definition::DunkM3x1y7S4ECgSL4Wkru:72",
            r#"{"ref":72,"origin":"DunkM3x1y7S4ECgSL4Wkru","signature_type":"CL","data":{"primary":{"n":"82730043187357279718335653276186066723813244865680356984530221947322656919740482532302246387936888766116887256097076953170218630332844353185028780127658814165522449835061954438190537301522646667997468474536735805281392103007884373870270813050354878058594736404488530861217888672332541236452858784206904498067504326289252573296829726183642730291965485922012669417558636463267841012387973405751871118653741168465507896617492544202194531932585542728241933216578617743307041199425901811703990596704064581229670691920644899224762774550835553834862012756882731989856767528955807239225515056404397885835776202568490216665437","s":"71026797257878935309170983450950471765035839828760065123335720323413925365892062317594144079666072402367984667837716999706658688896995850234803277947734032521338043949394949305076580128219959411123473506241134132748882541583082863580863696585209752636249540477578314498052790765574381349822219780659663228659519659227535764623158683548902935239675278878448684635584156256642899139913797198338559974346346347574537725648798844395511089910279213066775812725238667428112588275511738817682219659114529793152650297742141681931730490270365248899133874555047780847464159680240870281962608682174439611573427391240135897548334","rms":"10964843548699928985640907542427010193450453349982364607466583880902739335931344345619933697390904349356949255471142683331523921926900895350179767626401496704237071426710011838195240548157096370216094935970343641538163735066935778563842358319246355177896032021719400132056392661244782171478851204838993161865536316693396463017677188857754037247404187332333857116106842911941342829463958360380908381202340566426470270566862700296491953319058137790904601500663186607617542764021647650263861069894945982592554901813770640430394809689910579684191343536405446000769073373087533123009853204824329309055977543939735518573496","r":{"claim2":"68796303855560600116861115090483353851332391499443813429892720688496514117478750946407065436977340099767035451301209382010761035334924195509934308255550399382515032644159603482095996818991531808651879991597866186078500918257759528481748918946589923643440108855136721730396181248523551715042304444382809495486651071134426771342915652120363213972425218181620226861426442844836346120505399937171135779428243461505767455682252450811953380924754582535337719970440256000615594203436064777856497338626112033227532634188287674415555398328948773837242419585758245598864214573760071073474633943714923181542172036462866323539798","c2":"7330555144463370137564249878252158256874402442808950492217966397972579880056931399607929328314718601585304816932722787001960790788980817377443918899185474361825601225627953765462596452851582234047975167010526878870819295650102747020019885879512784983711628867220384199432751786571691297146825737152994656723621200986493583885341969837608658732600010669056250085079120462747527806133253284301726559882974939621509322463782757778250800193475066516542257377141774262434166801856910281268837547062714776918323353261103320857025645513866662425979557871240541532237944389844875756611439131571117419927440244363304990788677","a2":"41481448296257595539987751045823515208966972017105359798652240376258487704451825240745082417175260895450717090442850904131584259851974534166971037581735585923004606314899592179277462446869989638174105121451070653690654704637710894731622979252963559337090209603117358652069404965534126335004062199488369362416021619039643414806062128359809114761299238645481548783873312263813700570276752748570283464828528258128367587369420953925583870967923183694537896186570645364076005634044169294849972407321500849393207647040723693767565364600738100220323985016158499013349197649667623158147984281756979936771123530418268806676869","d2":"40376800953397693869808031086289799881711141389083400556745983840589368055586589969376042218921803686184716176782482345884070025315939115065428460033546936517119616671166101793965398715068350247280579680554546491127216094997456306556617719519575949314182904810065804839802027248637663864694778690377059281889576277787842768293438714335574294347675060562028206478227652027674778330313798409304892563819148491870869562163738634941653453966708542168427900216665314499440797818681370823170993983621185071380371289541269470645620690696776133769594222413758788678992141864113417134704287054327061696747018516811209306325747","b2":"20686081930067818666466815023782379851923077196271425865463846135862529512808216759965718023728689662214256901184719932365212549877547017465906257865680777751611327426786895327754243558121745835785687169436068091861592428514148676145254129956192761755554218348941500253455287633273190205391058263627502121344084178559843759730341166090534600868455537392865796225609442582923895037812110465605306543895133993656386658529320836957483664062776983348974993305907587596092931021149232341089727219247476073386253488334521475227651115425990647952732646513450916193478380369214314106047680869040370307518518726229426763871857"},"rctxt":"36550210260367809526599559615493628770004861191532468523416296038725527132127087279928521488614923739535408288969498685005638179183830309485147139637164607818357497022851752330475683229838627094179371430824325569206955895238512073392829772196507485950572301934844924646352083528692024244858956712829137234405829263477290860472141273089646684343917664745469614867597990447933937219860672057708832180431107882747429146170925256289339650349818021052479627110300018638318063811181290616788314588116313526164225316899225940336000266320021449949162733435599899861195881455284631001573985475311986071070652229715332620361156","z":"13917576851563210663670080158617511472671950108588762052019735862744900566530574180851800188167516444337606578392052672245337225553207845283293309204683817290331024148335544696158222656541711232776358171007496871020684866420684832355203922085278515726919265643572042984726071908002845842173395460072908578074275749224465487088960034769560842477750406372695151057643971447408929907899857443554689961049432309996282368024582732785832034871568415627541716676344225947487937739437514085591942263208353437980808079327691380700251910544599336609617080645191255107359257151147072399215020368683966894822585503110395624207476"},"revocation":null}}"#,
            "2018-01-22 17:09:52",
        ],
        //Claim 3
        [
            "claim_definition_private::EmapZ8H9S2qPp3JKyfr5z1:74",
            r#"{"secret_key":{"p":"175376972162281022380367007266903679805545681437797470948223181051318445550530588050595768143904377774160580139163842574750164436012415176422521848500414566783179062427933398687220832140080728450719460636014103519467310933445615184367033638378924658181005172417976294772431171340283769936169041285032448696713","q":"156504499011230284681044262397833298499746265961054298370236711678876988638077417907446408988711337763308458378172011812414413713874258585838782116111026729268885104155301270566697338099349636250233859556757080312770744013780587116179888786126415898288243229757876488799583360462280197567925353940528812193449"},"secret_key_revocation":null}"#,
            "2018-01-22 17:12:23"
        ],
        [
            "claim_definition::EmapZ8H9S2qPp3JKyfr5z1:74",
            r#"{"ref":74,"origin":"EmapZ8H9S2qPp3JKyfr5z1","signature_type":"CL","data":{"primary":{"n":"109789140665457085714916466101616078579686821932733655872241927448924187372159520230511538949872240436961882040827214594633719196606468880791200619360082818283496872281971805861122339438712617941661574555489986034066296907797088279251256509506422175888796784347483439615998819709200315815115739632148101410380790300794103055496147717872832229025369193066239885474861799335642412071471598579069213011447039808442263053432084009953269960305752459038441429851986041393957540913636012950434299203160112824853406832912295061994802494366029984353270402023482756439610370554870879497163291525385833409438934421124574067512873","s":"93672182104598870466607590909739101511314276704547605658537033734287410979828235106011329664042760884863258995384495725316032875797444311213805777968773374131827101422357970498031217148486595423733100062051325410763367052358676889441672710155500098342293278526307627240149634287614539015665361256984355832337897288581930778505199092446875725844442084724196179702404155186270556508715971202780043134608083197980374456824348254291677011507927127123307433354455956487784178995727028377214162679173625110564980164784352747293633697187862073003295621374384168211906633100096102407019338110338840909738786538148532229201157","rms":"109539914021820719768189687612099940804591908891405053904594948651600909031028186653166481171220379336361165198848386505602799267792722530957626916191318632756126701863528369025543128167227337442691316479539392574483285309192720420260927536144052610509168094526428888505733098386652309740953181219232762244968604573381444321965050913770716630698472722127340779516545093008753985842942213889135167860210139024255645494328267942147268696734306295009851727328984789421066094775719438626361102711732486989539729743736577344949190072538472003081884462541867307142549837246623398704948908755942373307874431953610413439015563","r":{"c3":"45500122689409460459216819271618191152863904072283239663719466401834572958853323418709946247968108420436821925371606995588102164205080683818785162072158321521884512544424718100161303002553853521719275280472859809885505567077074766057093325562803846798928077903085392556300083196921198717947840983261633764867398431494432586321631589022497129239910981144815668511808615150894388022520047081994610187760865799575401297176999906789130705804921418754717508744290735787945464928298145404472073084759937226172159268259214169451909128184650437096576033660397558827633153678432420206570042713028234993612556135389095793261494","a3":"65790287519505543377767837107220217674262001905838121125308976545063738202925665604283441036895587123908620153945268830735050023306283213094184529188306425147301226490692412556046241255761624241601130629343090645864680335042166116544654822567329603460190826358878434370342009497494269138219705609384894654278754381635439832037847177443602558201117430926743431782508435483370647967199353204845243541589515167181671397999782945885217457556722560413659739208887186165460998649073147390157058409122972817658106388418942285114166634082584963438822916078917562438271631697904100070188524821196640187530685742737833657586019","d3":"3441886195899401047635780730012568436398370052627912747256069086686664973566697397953267365158438827724806624471297877342688982016921109081228105875240684017887336948193480261332108577708305837019165278079091558181654681273533747910524379371730548942009422386100052001539371532295355017231442011851939809375650694278223906845687740651665892891191920150235166639455491007681006466977075823827430556039330551241018148230705724221474586879486757757542368165319060085500706540485964112201975757960031612266891317715046495928481250629575662404502396190912216506705541032466441926399634129665958688393794238559526193602686","b3":"19749690745897586626487208623998973738095275098556211548514308849218459243787847016743285870655882668852594706360211619940962122957268227758008545133868530533372516474231436482118916579503855586182210710035033075266816598486153037119874675234933853224933095961943045116360978392890579869759538571545034611175692205267640503836455996868301207567081569924107412871933530251367688795218348917576856534101977894386596817262868150982150374015698144370941926057815106473317866632986491562164980273085359179012816989707754649451718526515480363939054346999555706906690043837668563853857472791866389365844143729709978309319412","claim3":"44859439608472983835087203354721197511288445688118637615212033554103171057598149414910246816548279874232914727219635041180701461471729015988460712153878179632215807619556717785707333613406110798400747214461793072386482605716690780070781539681550899739615629055512854178813767523691663661115256826801186095075746706404850056904800671607647005585179433846375903764762004343616760367360593566005656614534041419797300345642574417838562484745717088596206074534030649089596833719898871337659771596187959787368528172644136700084836893579284791815117992350652293518802672373036493083539652637661336432903844559651514310984012"},"rctxt":"37778208541494576825356253509888314488182795688603526121071630425037966253367311110931041898875675813900935459167898265097372581683860530230249946758470544171759977571050619231081040914084668597137522561529510214231457061489213457033868378122696903037034243112573215372156596558248356679292127107133469838896700094464320791006114778359235213904394014426398306372946102388718625235517684460098445655027029229782345851143429624714549809347046885843524181122043094745353858547392031867256469582308456677967228572625792541862946195181039407357718604339826735100178541319736878556160460466131886492503309347143401075904543","z":"11424698398282298263929418832635281721997305622492884123140878196486631160065705024669828931398277960961954364846304372027218065922996353668537345453391262423840821508695335616363449030628568704334797662528318896735615657556768278133106424115124345189586034632707293318850915757107706601083950067392345644083318369820474180445488484469299972298842175099984581244643183101069115247645150983591277481131918744796643756731865703001861510947020766857364765043033275267316664585245343811657356940883750596763744448677938664735752944528414768746948535430252599813032572247609370659225460925402081101970831462596541447649148"},"revocation":null}}"#,
            "2018-01-22 17:12:23",
        ],
        // Claim 4
        [
            "claim_definition::2hoqvcwupRTUNkXn6ArYzs:22",
            r#"{"ref":22,"origin":"2hoqvcwupRTUNkXn6ArYzs","signature_type":"CL","data":{"primary":{"n":"95988816833977718321053908250488629570457514208068630867750478683940516331190512635343875556353861996520420942942764044177763002317208256863583692280833664390924596927347917276916509712265099683701719722190847264336011366791349646628075830228221107542544322913672429252134634395372199666668133488708754715906386228819303847415355653588070547345790507756751815270365039011442555013267500268911273838928496074646366967013006410177564457589181935498611626078606100640175127268290631713582389212871195358403592668156815115204909093182426970335081558255981914344710344119317989929074630230145638022976317221173764697186421","s":"62239136673167627400423120471558362740458209171638071978601918053566350183687450717956992748067512356899370528120135193319392151694058748777060748081293323091215103099387813131878931292003270092633409753870130426381678924026245559969559059962714007210659185095561238242139898811485236935881011836748903561459433779778161892475164468554655878585359186684311981090938613189604474762067065341487499103199801119116900325020066015355408081181975583856258510412824439820070724438199956292870224543214915058121622865521176513439841954457206050314635711702515741604006521885357490977012671983238692772732050068670317432119054","rms":"2666742394705868422197949070279883409319120284432003775492060510232976427038817653106315822802224834945991535639305682492345293889828389066920020784361847559346088721405588350403736328639678112687453643191645972753864555837116824170886546947152931560986163238196094020179916172762650563939600887215771633895234076357514206807140568263467898745036078268629871170965547759267217089200408183373560209245380839040472521799103156292656587016193867160956678985890522193369534824821432616210787583250746210963436574615123008203792318992499168295961682255891427304653899735468070666858394635493017671211372298428346694370136","r":{"address1":"79430106995260793105642076079163752606161347435237878678168203044689856051822547487766521563311680380753591909939480622687752539511101280052606606866629584321137355571906597893399076375179861217245568711517649222005785823632572289003993523491122898953296585841084813122075085098097356338488813716328441796484205489885849185984642169508079526567645828355730756526411908147442665312484850690225714502480915918839393044940407785782203869407374317891972832948898772951301643372985568565914280784081766369554450649995485801960050418761044734560643168899544753494595613916165018601714048428268251753045528994243087065144835","zip":"22615254879355764653958650478419607599831036235429432450152331935938535743623465642971741725376817628794671474016269970108464110269202618366187203073583413269450891057851388983903286090158743373372502765956052072615948851315985651162647206520535661255385577276988024064090606137548366392128706180770747410774135297692937003056030893838687872629072920391121220887197033968789644634843044406106556939756745198489147258162398860766157964749645169766410260222255289941570381220758288764725948145819227209530054486639704113062697253001037291479989539921868196758105947084202844813888484008565116517745895178211292455694733","state":"5389307291795023724767396590585051755492881239862831966011285708839846644667534729677314805974713243759733463689119279401855775452689115868477769023824603769488834865779417919635825920919975519104516947401633401985460129750314494342807379684704178680513110561610788646263003522590695544208887512032194707187894836570620414582795466298418842377750383651243769758829214009962185051608355599514279256896379204901072282321061082831337616871861533868887226814211876439835255986208696211885637396407217496465291231085562106868927260469043543489659790071945183660814540207128940863200709695858489503545193374804871313764617","city":"74293894871977367759060252704534345335218780070750078875399161307877509632220987359349558825164299759132753770718269112182715877390141990101932340866343914532776732207720227212332987373975484039824934448470702683132162518162474377092967237940469633226277271723192881037579252515172943950534199753098776099352368667398023761611922422044473986083577562168309539427651434633868951151558156078066862888877452828814049212303812161652181936924323630202737030127237808172796156569876764079540381483951067805130896942315889373571107898166154732259892131126166507045712723368578395992958246741207450949682368692070109832600250","address2":"62245102215581448501024795144133233272032362386257832116292842768642447199425235407152317927152360802393938767974847625947081778641332887450797640622194038530111791916923268259520523425958949520886626976593312640864134231960227939479094814565673466738383505704629110772101845490196382811902708763639214281775907141634474001554782825796701982840574757506850919691605980110873914124778342476385024907780564691749527899177013978495925954891042213505032471154249587217342488105382143964872162352317153795918703738602811294988020790400406830570899346692888402195222524206807060033262678435941237800852691616184732199699983"},"rctxt":"95650189971639859896612697168398518174555679984051423930197404587933150680782307635981876795701282260886569030531509518577472744227104768139773550071582998135328541386103474244900743088590008426912178650425903427917608878536633077924458612023417837982597475938736958656995406235396866427894427477541584899831302054536164023375230562375959591948316605322328045641788957437240691382565803311876039782656157169530004694059508455786187015509258876824806694556808250916064559543523070870263318805331754691870543174609221526847542058954563771715740763642121304736631910606989341821044450289456724644698365195414508996978377","z":"30931274405908239007890874385170967603061487779677432303190865674750965567572978740022706424175437722173963850488296625234382628771402047435950207188587229830735692967511122478369596260635807055949842658003257782625140032345552811734684259724489194465204904940357385656425260021068243335380180404373610916660761856588524604896207550324650177162424459728110262870433527487174136617579313419321121537041812997489371858831584038908195572117490767641756996884580893003145723858706935861376796546277590651278836498576606347036898057816575425746855202249031593540829669694382215623764119884221830708338999694320658810512371"},"revocation":null}}"#,
            "2018-01-19 20:56:49"
        ],
        [
            "claim_definition_private::2hoqvcwupRTUNkXn6ArYzs:22",
            r#"{"secret_key":{"p":"147994992483722226695796045011684235809309423781128508158803929784354160216609372391549068696035183653818336853597866730292371284529976761888221582212813626580505679228888172522950665475350345678188763591772773741254257464285253418496398049565928060194413224976216924914132317463912094242224097692749187043829","q":"162148757912426329580848488425481308197570783683521257574076212536320583242105942819686169604321953715281620067963105294861614807334103995842227210817175897159871997469149148693805426651335858938548729488829824519277006024203128120131839471735406633445070652347566232438111357984964864315788582900774238972559"},"secret_key_revocation":null}"#,
            "2018-01-19 20:56:49"
        ],

        //DID DunkM3x1y7S4ECgSL4Wkru
        [
            "my_did::DunkM3x1y7S4ECgSL4Wkru",
            r#"{"did":"DunkM3x1y7S4ECgSL4Wkru","verkey":"837gvxJ2pUuhCspWti7qiRAo2JKCKaHg4X1k9SuKemWY"}"#,
            "2018-01-19 21:26:11"
        ],
        [
            "key::837gvxJ2pUuhCspWti7qiRAo2JKCKaHg4X1k9SuKemWY",
            r#"{"verkey":"837gvxJ2pUuhCspWti7qiRAo2JKCKaHg4X1k9SuKemWY","signkey":"xt19s1sp2UZCGhy9rNyb1FtxdKiDGZZPP7Uk7LfAi6P2ZgcpNMagSN7K1TDc73gkBzdpqvUAmpPnnxRLqXeBZbv"}"#,
            "2018-01-19 21:26:11"
        ],
        //DID 2hoqvcwupRTUNkXn6ArYzs
        [
            "my_did::2hoqvcwupRTUNkXn6ArYzs",
            r#"{"did":"2hoqvcwupRTUNkXn6ArYzs","verkey":"vrWGArMA3toVoZrYGSAMjR2i9KjBS66bZWyWuYJJYPf"}"#,
            "2018-01-19 20:27:14"
        ],
        [
            "key::vrWGArMA3toVoZrYGSAMjR2i9KjBS66bZWyWuYJJYPf",
            r#"{"verkey":"vrWGArMA3toVoZrYGSAMjR2i9KjBS66bZWyWuYJJYPf","signkey":"xt19s1sp2UZCGhy9rNyb1FtxdKiDGZZPPWbEpqU41NEy3MmoeUuhq6T8F4JuRPgtj3hoT6BBjgnEF4bbhsR84NH"}"#,
            "2018-01-19 20:27:14"
        ],
        //DID EmapZ8H9S2qPp3JKyfr5z1
        [
            "my_did::EmapZ8H9S2qPp3JKyfr5z1",
            r#"{"did":"EmapZ8H9S2qPp3JKyfr5z1","verkey":"8WFnbdyQkZ7pZDX9FRCLbtH4bjy3AacWKPYkpxHCxaVq"}"#,
            "2018-01-19 20:27:15"
        ],
        [
            "key::8WFnbdyQkZ7pZDX9FRCLbtH4bjy3AacWKPYkpxHCxaVq",
            r#"{"verkey":"8WFnbdyQkZ7pZDX9FRCLbtH4bjy3AacWKPYkpxHCxaVq","signkey":"xt19s1sp2UZCGhy9rNyb1FtxdKiDGZZPP7Uk7LfAi6PvimpyRHkzenYNbW6eLf5DKWU9mWHFQMWVpMQc7Bq3EL3"}"#,
            "2018-01-19 20:27:15"
        ],
        //DID M7uZU89SUdsav7i4hVZtXp
        [
            "my_did::M7uZU89SUdsav7i4hVZtXp",
            r#"{"did":"M7uZU89SUdsav7i4hVZtXp","verkey":"By1CvKuLFRRdqMyGsmu8naVQQQfSH4MYna4K7d4KDvfy"}"#,
            "2018-01-24 16:24:20"
        ],
        [
            "key::By1CvKuLFRRdqMyGsmu8naVQQQfSH4MYna4K7d4KDvfy",
            r#"{"verkey":"By1CvKuLFRRdqMyGsmu8naVQQQfSH4MYna4K7d4KDvfy","signkey":"2X9g9MoQZszUqeWVR6mF2MfrFu8uCHQC74sbARGKbgYuxE1ejYWXxSFkdLM7HQgXwLGUoRc4h8KS39ehfKZXecmV"}"#,
            "2018-01-24 16:24:20"
        ],
        //DID V4SGRU86Z58d6TV7PBUe6f
        [
            "my_did::V4SGRU86Z58d6TV7PBUe6f",
            r#"{"did":"V4SGRU86Z58d6TV7PBUe6f","verkey":"GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL"}"#,
            "2018-01-19 20:27:14"
        ],
        [
            "key::GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL",
            r#"{"verkey":"GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL","signkey":"xt19s1sp2UZCGhy9rNyb1FtxdKiDGZZPQ4HE7gcnS8BV8xho5wQnCBjgpqBf522RWEYgPDL7rpn9GFz3XXgcaHk"}"#,
            "2018-01-19 20:27:14"
        ],
        //DID 6pLgr11JDGqSiheutZiqH5
        [
            "my_did::6pLgr11JDGqSiheutZiqH5",
            r#"{"did":"6pLgr11JDGqSiheutZiqH5","verkey":"4ArxwLx44BsoXYrjmhi3zykqVqegQZUCtmY6uS1Zme7W"}"#,
            "2018-01-19 20:27:14"
        ],
        [
            "key::4ArxwLx44BsoXYrjmhi3zykqVqegQZUCtmY6uS1Zme7W",
            r#"{"verkey":"4ArxwLx44BsoXYrjmhi3zykqVqegQZUCtmY6uS1Zme7W","signkey":"xt19s1sp2UZCGhy9rNyb1FtxdKiDGZZPP7Uk7LfAi6PFvkGKYmCiC4zCmGriGpCDyPCYsuGzy5276S6xk4epPTA"}"#,
            "2018-01-19 20:27:14"
        ],
        //DID 4QgA4TeP8cx6N6CbmydXgY
        [
            "my_did::4QgA4TeP8cx6N6CbmydXgY",
            r#"{"did":"4QgA4TeP8cx6N6CbmydXgY","verkey":"2rjuC1MjXeRzD2HXZP7GbC2Jp94ejpfL9y5fa8vzh3k5"}"#,
            "2018-01-19 20:27:15"
        ],
        [
            "key::2rjuC1MjXeRzD2HXZP7GbC2Jp94ejpfL9y5fa8vzh3k5",
            r#"{"verkey":"2rjuC1MjXeRzD2HXZP7GbC2Jp94ejpfL9y5fa8vzh3k5","signkey":"xt19s1sp2UZCGhy9rNyb1FtxdKiDGZZPP7Uk7LfAi6PXqwa4zvUyhMwTKV4j8Pi1QTC7fxa5SLErLvzVNQrFkL1"}"#,
            "2018-01-19 20:27:15"
        ],
        //DID QMJEenHYEihhAYD9NX9QJA
        [
            "my_did::QMJEenHYEihhAYD9NX9QJA",
            r#"{"did":"QMJEenHYEihhAYD9NX9QJA","verkey":"Dj91BVcEneik3XzUFECpVHi1JSWD8Sp5C1f9gwLF1ZBp"}"#,
            "2018-01-19 20:27:15"
        ],
        [
            "key::Dj91BVcEneik3XzUFECpVHi1JSWD8Sp5C1f9gwLF1ZBp",
            r#"{"verkey":"Dj91BVcEneik3XzUFECpVHi1JSWD8Sp5C1f9gwLF1ZBp","signkey":"xt19s1sp2UZCGhy9rNyb1FtxdKiDGZZPP7Uk7LfAi6QKAyR44gBpo8ERzkqsKqrHwjw6xgmxUdArVR8L1fdcAGJ"}"#,
            "2018-01-19 20:27:15"
        ],
        //DID FTdABb4y74p2eqZ2bkZtwy
        [
            "my_did::FTdABb4y74p2eqZ2bkZtwy",
            r#"{"did":"FTdABb4y74p2eqZ2bkZtwy","verkey":"8t5YHzniLT1smCE4SBvvifdgFvY8MGFDrETCfZeMUpiH"}"#,
            "2018-01-19 20:27:15"
        ],
        [
            "key::8t5YHzniLT1smCE4SBvvifdgFvY8MGFDrETCfZeMUpiH",
            r#"{"verkey":"8t5YHzniLT1smCE4SBvvifdgFvY8MGFDrETCfZeMUpiH","signkey":"xt19s1sp2UZCGhy9rNyb1FtxdKiDGZZPP7Uk7LfAi6QXZEKfMcGAeA8ijmrHTkFiYU8JwN1vW8QzmAWAZQWbP33"}"#,
            "2018-01-19 20:27:15"
        ],
        //DID QMJEenHYEihhAYD9NX9QJA
        [
            "my_did::HqnbxWG5KbRN6DTnaTVRPi",
            r#"{"did":"HqnbxWG5KbRN6DTnaTVRPi","verkey":"ABP94vkg6mLaYntBg1A4goAov5GfNXMCjonME3dmf8ge"}"#,
            "2018-01-19 20:27:16"
        ],
        [
            "key::ABP94vkg6mLaYntBg1A4goAov5GfNXMCjonME3dmf8ge",
            r#"{"verkey":"ABP94vkg6mLaYntBg1A4goAov5GfNXMCjonME3dmf8ge","signkey":"xt19s1sp2UZCGhy9rNyb1FtxdKiDGZZPP7Uk7LfAi6Qr5rHw5Kpj1Dc5RDPkT9gstonDajsgyJVNPSyaxphHeFg"}"#,
            "2018-01-19 20:27:16"
        ],
        //DID Bv16f26bC9siA4G69MBNzZ
        [
            "my_did::Bv16f26bC9siA4G69MBNzZ",
            r#"{"did":"Bv16f26bC9siA4G69MBNzZ","verkey":"6x1RzzkXxN1zozTrzNRcdX6qMp9GvQxVr33FrDQFHbyy"}"#,
            "2018-01-19 20:27:16"
        ],
        [
            "key::6x1RzzkXxN1zozTrzNRcdX6qMp9GvQxVr33FrDQFHbyy",
            r#"{"verkey":"6x1RzzkXxN1zozTrzNRcdX6qMp9GvQxVr33FrDQFHbyy","signkey":"xt19s1sp2UZCGhy9rNyb1FtxdKiDGZZPP7Uk7LfAi6R65nxMw5HNednuhaVHzbY1qYKupFoz4ysxffD541wS4oH"}"#,
            "2018-01-19 20:27:16"
        ],
        //DID YQkSbpeC2PyycfLieXsaD5
        [
            "my_did::YQkSbpeC2PyycfLieXsaD5",
            r#"{"did":"YQkSbpeC2PyycfLieXsaD5","verkey":"J7v5TnKnWaus5BFw8FHmcNs9RwWzAgw6Q9iK77PHuMxw"}"#,
            "2018-01-19 20:27:16"
        ],
        [
            "key::J7v5TnKnWaus5BFw8FHmcNs9RwWzAgw6Q9iK77PHuMxw",
            r#"{"verkey":"J7v5TnKnWaus5BFw8FHmcNs9RwWzAgw6Q9iK77PHuMxw","signkey":"xt19s1sp2UZCGhy9rNyb1FtxdKiDGZZPP7Uk7LfAi6RaV1yKWQ8ivGRjy9yf3dzBv7AESGqyZ2ZzqHbBsRkZn2X"}"#,
            "2018-01-19 20:27:16"
        ],
    ];

    #[ignore]
    #[test]
    fn test_creating_claim_def_dependencies() {
        let wallet_name = "a_test_wallet";
        self::cxs::utils::logger::LoggerUtils::init();
        self::cxs::settings::set_defaults();

        pool::open_sandbox_pool();

        wallet::init_wallet(wallet_name).unwrap();
        let wallet_handle = wallet::get_wallet_handle();

        let (issuer_did, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(DEMO_ISSUER_PW_SEED)).unwrap();
        SignusUtils::create_and_store_my_did(wallet_handle, Some(DEMO_AGENT_PW_SEED)).unwrap();

        let data = r#"{"name":"Home Address","version":"0.1","attr_names":["address1","address2","city","state","zip"]}"#.to_string();
        let schema_handle = create_new_schema("1".to_string(), "name".to_string(), issuer_did.to_string(), data).unwrap();

        let schema_seq_no = get_sequence_num(schema_handle).unwrap();

        create_new_claimdef("anything".to_string(), "anything".to_string(), schema_seq_no, issuer_did, false).unwrap();
    }

    #[ignore]
    #[test]
    fn test_putting_claim_def_dependencies() {
        use std::path::Path;
        use self::rusqlcipher::Connection;
        use std::env::home_dir;

        self::cxs::utils::logger::LoggerUtils::init();
        self::cxs::settings::set_defaults();
        let wallet_key = String::from("");
        let home = home_dir().unwrap();
        let wallet_name = "my_real_wallet";
        let wallet_dir = format!(".indy_client/wallet/{}/sqlite.db", wallet_name);
        let wallet_dir = Path::new(&wallet_dir);
        let wallet_db = home.join(wallet_dir);

        if wallet_key.len() > 0 {
            self::cxs::settings::set_config_value(self::cxs::settings::CONFIG_WALLET_KEY, &wallet_key);
        }

        self::cxs::utils::wallet::init_wallet(wallet_name).unwrap();

        let connection = Connection::open(wallet_db.as_path()).unwrap();

        if wallet_key.len() > 0 {
            connection.execute(&format!("PRAGMA key='{}'", wallet_key), &[]).unwrap();
        }

        for entry in ENTRIES {
            connection.execute("INSERT OR REPLACE INTO wallet VALUES (?,?,?)", &[&entry[0].to_string(), &entry[1].to_string(), &entry[2].to_string()]).unwrap();
        }
    }
}