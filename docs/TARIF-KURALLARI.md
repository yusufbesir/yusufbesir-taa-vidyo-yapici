---
description: TAA Vidyo Yapıcı "Tarif et" çevrimdışı kural listesi (düz Türkçe belge). Kaynak: kurallar/tarif-kurallari.json (otomatik üretilir; elle düzenlemeyin, kural eklemek için programda "Bunu kural olarak öğret" ya da JSON dosyasına ekleme). Sahip Yusuf; yazan [Claude#ebd691, 2026-11-16]. Silme yok.
---
# Tarif et: çevrimdışı kural listesi

Toplam 708 ifade, 234 kural. Her ifade bir katalog adına bağlıdır; ek ve çekim toleranslıdır (zıpla/zıplayarak/zıplatarak), ı-i ve ş-s gibi harf farkı yok sayılır, büyük/küçük harf fark etmez. 3 harften kısa ifadeler tam kelime olarak aranır; 4 harf ve üstü ifadeler ek alabilir.

## Nasıl çalışır
- Metin tırnak içinde ("...") ya da BÜYÜK HARFLE yazılır; kalan kelimeler kural olarak okunur. Hiçbiri yoksa kalan kelimelerden metin tahmin edilir (önizlemede "tahmin ettim" yazar).
- Sıra bağlaçları sahneyi böler: önce, sonra, ardından, derken, en son, daha sonra, son olarak. Cümle sonu (. ; ! ?) ve virgül de cümlecik ayırır; metni olmayan cümlecik önceki sahneye (yoksa sonrakine) özellik ekler.
- Süre: "3 saniye", "2,5 sn", "iki saniye", "yarım saniye" (en az 1,5 sn'ye çekilir), "bir buçuk saniye", kısa (3 sn), uzun (7 sn), çok kısa (2 sn), çok uzun (10 sn).
- Geçiş kuralları yalnız cümlede "açılış", "kapanış" ya da "geçiş" geçerse uygulanır.
- %30, "3 taksit", "1500 TL" gibi desenler metin yoksa sahne metni olur.
- Anlaşılmayan kelime SORULMAZ: önizlemede "anlaşılmayan kelimeler" olarak vurgulanır; "Bunu kural olarak öğret" ile kalıcı eşleme eklenir (kullanıcı kuralları %USERPROFILE%VideosTAA Vidyo Yapıcı	arif-kurallarim.json).

## Giriş animasyonu (242 ifade)
| İfadeler | Katalog adı |
|---|---|
| zıpla, zıplat, hoplat, hopla, sıçrayarak gel | zipla |
| dön, dönerek, döndür, dönsün, çevrilerek gel | donerek |
| yandan gir, yandan kay, yan taraftan | yandan |
| zoom, yakınlaş, büyüyerek gel, yaklaş | zoom |
| alttan, aşağıdan, aşağıdan yukarı, alttan gelsin | alttan |
| üstten, yukarıdan, yukardan, tepeden | ustten |
| belir, görün, ortaya çık, yavaşça görün, belirsin | belir |
| elastik, lastik gibi, esneyerek | elastik |
| daktilo, yazı makinesi, harf harf | daktilo |
| toplan, toplanarak, bir araya gel | toplan |
| düş, düşerek, düşsün, düşüş | dusus |
| sağdan, sağ taraftan, sağdan gelsin | sagdan |
| soldan, sol taraftan, soldan gelsin | soldan |
| merdiven, basamak, basamak basamak | merdiven |
| çevir, çevirerek | cevir |
| salıncak, sallanarak gel | salincak |
| büyülü, sihirli, sihir gibi | buyu |
| patla, patlayarak, patlasın, patlama | patla |
| sars, sarsıl, sarsıntı, sarsılarak | sarsil |
| dalga, dalgalanarak, dalgalı gir | dalga |
| spiral, sarmal, helezon | spiral |
| derinleş, derinleşerek | derinles |
| uzaktan, uzaktan gel, uzaktan yaklaş | uzaktan |
| sinema, sinematik, film gibi | sinema |
| nabız, nabız gibi, atarak | nabiz |
| neon, neon gibi yan | neon |
| yavaş belir, yavaşça belir, usulca belir, usul usul belir | yavas-belir |
| yüksel, yükselerek, yükselsin | yuksel |
| yıldız gibi, yıldızlı giriş | yildiz |
| mercek, mercekle | mercek |
| kapıdan, kapı gibi, kapı aç | kapi |
| fırlat, fırlayarak, fırlasın | firlat |
| yaprak gibi, yaprak gibi düş | yaprak |
| sıçra, sıçrayarak, sıçrasın | sicra |
| yıldırım, şimşek, şimşek gibi | yildirim |
| kuyruklu, kuyruklu yıldız gibi | kuyruklu |
| hızlı, çabuk, hızla, hızlıca | hizli |
| ağır, ağır ağır, ağırdan | agir |
| eğil, eğilerek, eğilsin | egilme |
| fırtına, fırtına gibi | firtina |
| dev, devasa, dev gibi | dev |
| mekik, mekik gibi | mekik |
| uzay, uzaydan, uzay gibi | uzay |
| zoom in, içeri zoom, içe zoom | zoom-in |
| zoom out, dışarı zoom, uzaklaşarak gel | zoom-out |
| flip, ters yüz, ters yüz ol | flip-x |
| tur at, turlayarak, tur | tur |
| yuvarlan, yuvarlanarak, top gibi | yuvarlan |
| ışık hızı, ışık hızıyla, ışık hızında | isik-hizi |
| sarkaç, sarkaç gibi | sarkac |
| mürekkep, mürekkep gibi, yayılarak | murekkep |
| netleş, netleşerek, odaklan, odaklanarak | odak |
| asansör, asansör gibi | asansor |
| iniş, insin, aşağı insin, inerek | inis |
| kalp atışı, kalp atışı gibi | kalp-atisi |
| zikzak, zigzag | zikzak |
| balon gibi yüksel, balon gibi yüksel | balon-yukselis |
| kumaş, kumaş ser, kumaş gibi | kumas-ser |
| makas, makasla, makas gibi | makas-acil |
| metre, mezura, mezura gibi | metre-kay |
| gece yarısı | gece-yarisi |
| asılı, askıda, asılarak | asil-yukari |
| etiket sallan, etiket gibi, sallanan etiket | etiket-sallan |
| vitrin, vitrin gibi | vitrin-kay |
| halat, halatla, halat gibi | halat-sallan |
| dikiş, dikiş izi, dikişli | dikis-izi |
| omuz, omuz gibi eğil | omuz-egil |
| elmas gibi, elmas parla, elmas gibi parla | elmas-parla |
| açılış perdesi, perde açılır gibi, perdeyle gel | acilis-perdesi |
| kuyruk, kuyruklu | kuyruklu |
| yavaşça soldan, yavaş soldan | yavas-sol |
| yavaşça sağdan, yavaş sağdan | yavas-sag |
| hızlıca soldan, hızlı soldan | hizli-sol |
| hızlıca sağdan, hızlı sağdan | hizli-sag |
| yaylanarak, yaylan, yay gibi, yaylanarak gel | yaylan-alt |
| zoomla soldan, zoomlu soldan | zoomlu-sol |
| zoomla sağdan, zoomlu sağdan | zoomlu-sag |
| kayarak soldan, soldan kayarak | kayar-sol |
| kayarak sağdan, sağdan kayarak | kayar-sag |
| kayarak yukarıdan, yukarıdan kayarak | kayar-ust |
| kayarak aşağıdan, aşağıdan kayarak | kayar-alt |
| nefes, nefes al gibi | nefes-gir |
| tek tek belir, harfler tek tek | tek-tek-belir |
| ince ince belir, incecik belir | ince-belir |
| yıldız gibi dön, yıldız dönüş | yildiz-donus |
| zig zag, zik zak | zikzak |
| balon gibi yüksel, balonla yüksel | balon-yukselis |

## Çıkış animasyonu (44 ifade)
| İfadeler | Katalog adı |
|---|---|
| yukarı çık, yukarı kay, yukarı doğru çıksın, yukarı uç, yukarı çıksın | yukari |
| aşağı in, aşağı kay, aşağı çık, aşağı çıksın | asagi |
| dönerek çık, dönerek çıksın, dönerek kaybol | donerek |
| sön, sönsün, sönerek, kaybol, kaybolsun, yok ol, kararsın, karar | sondur |
| küçül, küçülsün, küçülerek, küçülerek çık | kucul |
| silinsin, silinerek, silinerek çık | sil |
| dağıl, dağılsın, dağılarak | dagil |
| uçarak çık, uç git, uçarak kaybol | ucarak |
| düşerek çık, düşerek kaybol | dusus |
| patlayarak çık, patlayarak kaybol | patla |
| yıldırım gibi çık | yildirim |
| uzaklaşarak çık, uzaklaşarak kaybol | zoom-out |
| hızlıca çık, hızla çık | hizli |
| yavaş yavaş sön, yavaşça kaybol | yavas-belir |

## Efekt (125 ifade)
| İfadeler | Katalog adı |
|---|---|
| parlasın, parlama, ışık geçsin, ışık süpürsün, ışıltı, parlayan | parlama |
| pırıltı, pırıl pırıl, simli, parıltı, yıldız parıltısı | pirilti |
| çerçeve, çerçeveli, kenarlık, kenar çizgisi | cerceve |
| çift çerçeve, iki çerçeve, çift kenarlık | cift-cerceve |
| parçacık, parçacıklar, toz, tozlar | parcacik |
| kıvılcım, kıvılcımlar, kıvılcımlı, ateş kıvılcımı | kivilcim |
| kar, karlı, kar yağsın, kar yağışı, kar taneleri | kar |
| yağmur, yağmurlu, yağmur yağsın | yagmur |
| halka, halkalar, yayılan halka | halka |
| kayan yıldız, yıldız kaysın, meteor, göktaşı | kayan-yildiz |
| çiçek yağmuru, çiçek, çiçekler, çiçek yağsın | cicek-yagmuru |
| papatya, papatyalar | papatya |
| yaprak, yapraklar, sonbahar yaprağı, yaprak yağsın | yaprak |
| kelebek, kelebekler, kelebekli | kelebek |
| uğur böceği, uğur böcekleri | ugur-bocegi |
| arı, arılar, bal arısı | ari |
| ateş böceği, ateş böcekleri | ates-bocegi |
| kalp, kalpler, kalpli, kalp yağmuru | kalp |
| yıldız yağmuru, yıldızlar, yıldız yağsın | yildiz-yagmuru |
| konfeti, konfetili, konfeti patlat | konfeti |
| balon, balonlar, balonlu, uçan balon | balon |
| havai fişek, havai, fişek, havai fişekler | havai-fisek |
| pırıltı bulutu, simli bulut, parıltı bulutu | pirilti-bulutu |
| ince çerçeve, ince kenarlık | cerceve-ince |
| şerit, zemin şerit, şeritli zemin | zemin-serit |
| yıldızlı zemin, yıldızlı arka plan | zemin-yildizli |
| nabız zemin, zemin nabız, atan zemin | zemin-nabiz |
| sütun, sütunlu zemin, sütunlar | zemin-sutun |
| iki çizgi çerçeve, çift çizgili çerçeve | cerceve-cift |
| köşe çerçeve, köşeli çerçeve, köşe süsü | cerceve-kose |
| noktalı çerçeve, nokta çerçeve | cerceve-nokta |
| neon çerçeve, neon kenarlık | cerceve-neon |
| yıldızlı çerçeve, yıldız çerçeve | cerceve-yildiz |
| dalgalı çerçeve, dalga çerçeve | cerceve-dalga |
| ışıklı çerçeve, ışıklı kenarlık | cerceve-isik |
| altın çerçeve, altın rengi çerçeve, altın kenarlık | cerceve-altin |
| şeritli çerçeve, şerit çerçeve | cerceve-serit |
| gradyan, gradyanlı zemin, renk geçişli zemin | zemin-gradyan |
| dalgalı zemin, dalga zemin | zemin-dalga |
| ızgara, ızgaralı zemin, kareli zemin | zemin-izgara |
| plazma, plazma zemin | zemin-plazma |

## Renk / palet (152 ifade)
| İfadeler | Katalog adı |
|---|---|
| altın, altın rengi, altın renkli, gold, altınsı | altin |
| beyaz, beyaz renk, kısık beyaz, beyazlı | beyaz |
| sarı, sarı renk, sarılı | sari |
| turuncu, turuncu renk, turuncuyla | turuncu |
| cyan, camgöbeği, cam göbeği, siyan | cyan |
| magenta, fuşya, fuşya rengi | magenta |
| lime, fıstık yeşili, limon yeşili | lime |
| altın gece, siyah altın, koyu altın | altin-gece |
| sıcak beyaz, krem, krem rengi, bej, fildişi, ekru | sicak-beyaz |
| buz mavisi, buz cyan, buz rengi, buz gibi | buz-cyan |
| pembe, pembe renk, pembe parıltı, pembemsi | pembe-parilti |
| gök mavisi, açık mavi, bebek mavisi, mavi gök | gok-mavisi |
| mor altın, eflatun, menekşe, menekşe rengi | mor-altin |
| neon yeşil, zümrüt, zümrüt yeşili, parlak yeşil | neon-yesil |
| kırmızı altın, bordo, vişne, vişne çürüğü, bordo renk | kirmizi-altin |
| turuncu gece, koyu turuncu | turuncu-gece |
| mercan, mercan rengi, somon, somon rengi | mercan |
| gün batımı, gün batımı renkleri, ateş rengi, alev rengi | gun-batimi |
| okyanus, deniz mavisi, deniz rengi, lacivert, petrol mavisi | okyanus |
| mor ışık, lila, lila rengi | mor-isik |
| lavanta, lavanta rengi | lavanta-altin |
| nane, nane yeşili, mint, nane rengi | nane |
| şeftali, şeftali rengi | seftali |
| limon, limon sarısı, limon rengi | limon |
| bakır, bakır rengi, bronz, bronz rengi | bakir |
| gümüş, gümüş rengi, gümüş mavisi, gri, çelik, çelik rengi | gumus-mavi |
| gül kurusu, gül, toz pembe, gül rengi | gul-kurusu |
| yeşil altın, haki, haki yeşil | yesil-altin |
| buz pembe, buz pembesi | buz-pembe |
| beyaz cyan, beyaz camgöbeği | beyaz-cyan |
| beyaz pembe, beyaz ve pembe | beyaz-pembe |
| cyan vurgulu, cyan vurgu | cyan-vurgu |
| magenta gece, koyu magenta | magenta-gece |
| altın cyan, altın ve cyan | altin-cyan |
| pembe altın, pembe ve altın | pembe-altin |
| tek altın, düz altın | tek-altin |
| sarı lacivert, sarı ve lacivert | sari-lacivert |
| kırmızı, kırmızı renk, kırmızılı, al renk, saf kırmızı, parlak kırmızı | hex-ff2b2b |
| mavi, mavi renk, koyu mavi, parlak mavi, mavili | hex-2f7bff |
| yeşil, yeşil renk, yeşilli | hex-22c55e |
| mor, mor renk, morlu | hex-a855f7 |
| turkuaz, turkuaz rengi | hex-14d8c4 |
| kahverengi, kahve rengi, kahve | hex-8b5a2b |
| hardal, hardal rengi, hardal sarısı | hex-d4a017 |
| şeker pembe, açık pembe | hex-ff7ab8 |
| açık gri, duman rengi | hex-b0b0b0 |

## Yazı tipi (74 ifade)
| İfadeler | Katalog adı |
|---|---|
| inter, sade yazı, modern yazı, düz yazı | inter |
| anton, dar kalın, afiş yazısı, afiş harfi | anton |
| archivo, geniş yazı | archivo |
| poppins, yuvarlak yazı, yumuşak yazı | poppins |
| barlow, dar yazı | barlow |
| slab, kalın tırnaklı, tırnaklı yazı | slab |
| russo, teknik yazı, mühendis yazısı | russo |
| fjalla, yüksek dar yazı | fjalla |
| bangers, çizgi roman, çizgi roman yazısı, komik yazı | bangers |
| bebas, bebas neue, ince uzun yazı | bebas |
| bungee, tabela yazısı, tabela harfi | bungee |
| bungee içi boş, içi boş yazı, çizgili yazı | bungee-ic |
| askeri yazı, stencil, şablon yazı | black-ops |
| audiowide, fütüristik yazı, teknoloji yazısı | audiowide |
| righteous, dik yazı | righteous |
| kanit, spor yazısı | kanit |
| paytone, şişkin yazı | paytone |
| sigmar, kalın yuvarlak, şişman yazı | sigmar |
| staatliches, poster yazısı | staatliches |
| saira, dar teknik yazı | saira-dar |
| rubik, sabit aralıklı yazı | rubik-mono |
| yeseva, şık yazı, zarif yazı, lüks yazı | yeseva |
| dm serif, klasik yazı, serif yazı | dm-serif |
| abril, gazete yazısı, gazete harfi | abril |
| lobster, el yazısı, el yazısı gibi | lobster |
| pacifico, imza yazısı, imza gibi, kaligrafi | pacifico |

## Açılış ve kapanış geçişi (58 ifade)
| İfadeler | Katalog adı |
|---|---|
| perde, perdeli, perde ile, perdeyle | perde |
| tv, tüplü tv, eski tv, televizyon, televizyon gibi | tv |
| kapı, ortadan açılan kapı, kapı gibi | kapi |
| iris, daire iris, daire, daire gibi | iris |
| slayt, slayt kapağı, üstten slayt | slayt |
| fade, karartma, kararma, yumuşak geçiş, yumuşak karartma, soluklaşma | fade |
| neon, neon yanması | neon |
| silme, soldan sağa silme, sil | sil |
| jaluzi, jaluzi gibi, panjur | jaluzi |
| mozaik, mozaik gibi, karelere ayrılma | mozaik |
| dalga, dalgalı kenar | dalga |
| çubuk, çubuklar, dikey çubuk | cubuk |
| elmas, baklava, eşkenar dörtgen | elmas |
| kare, dikdörtgen, dikdörtgen iris | kare |
| saat, yelkovan, saat gibi | saat |
| çapraz, çapraz silme | capraz |
| damalı, dama tahtası, dama | damali |
| yanlar, iki yandan, iki yandan kapan | yanlar |

## Görünüm (13 ifade)
| İfadeler | Katalog adı |
|---|---|
| bina, binada, binaya, binanın, cephe, cephede, bina cephesi, bina ekranı, bina üzerinde | bina |
| düz görünüm, düz ekran, normal görünüm, düz | duz |

