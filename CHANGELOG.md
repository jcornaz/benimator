# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## Unreleased

<csr-id-d80e27892d13345b35255f6ea4c6f0d722780642/>
<csr-id-2635bc539115d879e4978b39f934a4c690d8152d/>
<csr-id-e5b3859748785255504708eb5fe4b87197079a3a/>
<csr-id-ebcaf2e12d6791076bb43f9d23d5f1be1c8a4a1c/>
<csr-id-944f1f225d2ed83273dd82ac64269c092a119dbb/>
<csr-id-18e8e98fc9a6f856fc929b52a37a27db95883bed/>
<csr-id-e5fc39fa02dd017fd44aa7bb8e7e1bc6dd40ae5a/>
<csr-id-1d4af4112e2dceff574ec881fa349856d97b9f72/>

### New Features (BREAKING)

 - <csr-id-0f833620082906210ab94c45ff3f13434c905979/> Public `update` function for the state
   The state is no longer automatically reset after the last frame of a `run_once` animation.
   So one must explicitly reset the state if they desire to restart the animation later.

### Chore (BREAKING)

 - <csr-id-d80e27892d13345b35255f6ea4c6f0d722780642/> Remove `AnimationPostUpdateSystem`
   It had no effect. This may be reintroduced (with effect) if requested.

### Refactor (BREAKING)

 - <csr-id-2635bc539115d879e4978b39f934a4c690d8152d/> No longer require animation to get the sprite index

### Documentation

 - <csr-id-1290d5c505246815dda6d025673a599bec37083f/> Fix incorrect 'since' clause for a deprecation

## v4.0.0-alpha.6 (2022-07-17)

<csr-id-02918b10c515fefcfda0819bdb3a38e27d79fd08/>
<csr-id-4b5bc45f9b62226512900ee736ba2708310304b5/>
<csr-id-47f13304be05f1a004ce8645a0f7f4ecd80b37b5/>
<csr-id-a32b1ad15dbbc0065d2bde20b886eebd60fb9fd6/>
<csr-id-0b8f79bcef66fea29319bf6aad364510bc9ff852/>
<csr-id-d15c32905856fb7da846eaa7d1f7114e475a91f2/>
<csr-id-ee9c0b1b192876b1ad376f8ef5e44d343b4207b2/>

### New Features

 - <csr-id-4b5bc45f9b62226512900ee736ba2708310304b5/> add `sprite_frame_index` method to state

## v4.0.0-alpha.5 (2022-07-16)

<csr-id-23502c8f2f286f4d9ac7e9c332158c84ee6babe1/>
<csr-id-f5c8f73f7a38ad989270a9517929416d381504af/>
<csr-id-b373bdae6322344d19a6c52ce52bc3962ce707bf/>
<csr-id-2a42ea5f3533ea8712026e8625c5993410f4f05f/>

### Refactoring

 - <csr-id-b373bdae6322344d19a6c52ce52bc3962ce707bf/> Remove direct dependency on `bevy_utils`

### Refactoring (BREAKING)

 - <csr-id-2a42ea5f3533ea8712026e8625c5993410f4f05f/> make `bevy_reflect` optional (behind `bevy-07` feature)


## v4.0.0-alpha.4 (2022-07-15)

<csr-id-8de593116cfbe838ca9dcffe0e1a552a6e6f50e1/>
<csr-id-75aa3347497bcb3aa30cfa208507b47e59f9c8a0/>

### Refactoring

 - <csr-id-75aa3347497bcb3aa30cfa208507b47e59f9c8a0/> make dependency `bevy_asset` optional (behind `bevy-07` feature flag)

## v4.0.0-alpha.3 (2022-07-14)

<csr-id-878af470cb350c262dd946f150eae22729e0efc9/>
<csr-id-a77893b52e4aa0a6e5ed98d5aeda2164d0fc99d7/>

### Refactoring (BREAKING)

 - <csr-id-a77893b52e4aa0a6e5ed98d5aeda2164d0fc99d7/> Use a single feature flag (`bevy-07`) for bevy integration

## v4.0.0-alpha.2 (2022-07-13)

<csr-id-c71a0dc88068559816fc9685351338316dd81062/>
<csr-id-f743445f66f8b53b9a8a89362bcafe585a6692a7/>

### Refactoring (BREAKING)

 - <csr-id-f743445f66f8b53b9a8a89362bcafe585a6692a7/> Make the integration with `bevy_sprite` optional and disabled by default

## v4.0.0-alpha.1 (2022-07-13)

<csr-id-1eed99ee8e9768f2b892222e61480fbd2226bd9c/>
<csr-id-e1fe1be9a597b515f94468273c77a9d3732d786e/>
<csr-id-5c8c0ffe24beeaf52ce56626efec2f24ddba6220/>
<csr-id-d77d2940dac0f040b5a7bd81f044f08e2d610d9e/>
<csr-id-8a8b16195556f4f6bd6ecdbc9c10e9095aa675fa/>
<csr-id-5da3509c492aeef1d7e848796882d2f1a4aafdbb/>
<csr-id-5cb1eff8fdfc2782b8566b362ae1ba60c72861e8/>
<csr-id-cb54d07320135417e3d4e1eb721ffffe4102e906/>
<csr-id-68ec6d9de6c2babb98171345ab55fa43e8314be0/>

This marks the beginning of the development of the next major version of benimator (version 4.0.0), for which I will try to make all bevy dependencies entirely optional so that I untie benimator's API stability from bevy's.

See the [related decision](https://github.com/jcornaz/benimator/blob/main/doc/adr/0002-optional-bevy-dependency.md) for more information.

### Refactoring (BREAKING)

 - <csr-id-68ec6d9de6c2babb98171345ab55fa43e8314be0/> Make the integration with `bevy_app` optional and disabled by default

## 3.7.0 (2022-07-17)

### New Features

 - <csr-id-e1249d62a7a0f2a63e401d44288295c81aaa5693/> rename state's `current_frame_index` to `animation_frame_index` (old name is deprecated)

## v3.6.1 (2022-07-01)

<csr-id-b6745f834355f519fe8692271c249c27297ec601/>
<csr-id-5602822ad4ab02bb69af18a5e7fd1015d076b232/>
<csr-id-4f5f6578ae06f112919a11d41c55c8e65c7a4f86/>

### Bug Fixes

 - <csr-id-3c025c07b1e417900524434f1bdc5cec97f332f2/> Delayed animation start when switching to a new animation

## v3.6.0 (2022-06-22)

### New Features

 - <csr-id-1201237dd81111584bbb05437a7995c0a181a504/> **unstable-load-from-file** `yaml` and `ron` feature flags to avoid pulling unnecessary dependencies

## v3.6.0-alpha.1 (2022-06-22)

<csr-id-30941ff0eeb922346ae7daf9f1c3860d1000f260/>
<csr-id-d59dc5d08c2e694a13a9b70caabfd17d642d748d/>
<csr-id-b65aceaa9f7aa4da002eb391e2dc0f75faaa1c53/>

### New Features

 - <csr-id-749661aec693e4b26fd549c91d6121d3c17fab32/> **unstable-load-from-file** Implicit 'Some' for ron files

### Chore

 - <csr-id-b65aceaa9f7aa4da002eb391e2dc0f75faaa1c53/> Require rust 1.60

## v3.5.0 (2022-06-22)

<csr-id-62d638b7fd3731ecb7578f2a913c8609db1a1794/>
<csr-id-7595217cd5cde4699487d6b64e791d68a2bf14c2/>
<csr-id-1af0d436d58759b5dead811bce3d196fc2dad2ad/>
<csr-id-a4dbd92b907efbda90bbd197b954d6f2a52f1106/>
<csr-id-286e48b9d265562561ca2c1e1c69b1618777785a/>

### New Features

 - <csr-id-7c2a5007dc06eed6e8d0832a6b3705bebffd1626/> Play animation with custom playback speed

## v3.4.1 (2022-06-21)

### Documentation

 - <csr-id-1194514436332b4673859f56e1f87142ffc0d72b/> **unstable-load-from-file** Mention support of ron files

## v3.4.0 (2022-06-20)

<csr-id-fce68d48753ff47ae535c3d91df3b6483f5e8ea5/>
<csr-id-e2e4907eb0dfaa68a345ae6169bf51a5d9315a08/>

### New Features

 - <csr-id-59bb02541886dd83ede7d7bb5e987397d0691900/> **unstable-load-from-file** use snake_case for properties and CamelCase for enums
 - <csr-id-b5288acd3e6d99fddf136443e8b4a62d5a66e752/> **unstable-load-from-file** load animation files with `.ron` extension
 - <csr-id-44b58bc0bf14b0ed97f747e01feeb9c93af36225/> **unstable-load-from-file** add `from_ron_str` and `from_ron_bytes` for `SpriteSheetAnimation`

## v3.3.0 (2022-06-15)

<csr-id-9c97f13e4a8ac7dd7443998840fbb0cb4492da42/>

### New Features

 - <csr-id-c0c6b3a62935abadf031514b008ce912c1b95fed/> **unstable-load-from-file** Shorter file format when all frames have the same duration

## v3.2.0 (2022-06-06)

<csr-id-6462f4286e4d99829ff92beb3c27455470cdea95/>
<csr-id-74559f92d1ba90c804f82c554d3ff3f06b82193f/>
<csr-id-5826aa165c2461975cc938fc4f023a402de7be40/>
<csr-id-eeacdf143ab259e0fd50bca003fe1af121eb5eea/>
<csr-id-2dc914ec213a9bbdc570fe9a8931af89e0325b30/>
<csr-id-fe78d9de34e2db02ea9ee00839f6b3f9291aa54d/>

### New Features

 - <csr-id-4db7d9c63dfd4955fc25d6168961a9ebb6c1a94f/> **unstable-load-from-file** Use a more yaml-y syntax for mode 'repeat-from'

### Documentation

 - <csr-id-44484dbdbc342e511cd36c1944f9a928cf2347ab/> Show members behind feature-flags

## v3.1.1 (2022-06-06)

<csr-id-fb1df3ffd782cdd466f2a909dd1177a575a41471/>
<csr-id-3afa0b70b17a30b58a1884efdacc5f91e903f543/>

### Bug Fixes

 - <csr-id-a15bf97d9fce3cd166478d963b90022a9956511e/> **unstable-load-from-file** Crash when parsing zero duration

## v3.1.0 (2022-06-05)

<csr-id-829cb424d2de6b0cb939d22d8eb662a8eebac5e2/>
<csr-id-2527f070abdaf264a3a8d85003e7c2467b5f1dc1/>
<csr-id-ed10f6c2b2ad207ebbacc5038c6734ae00e37719/>
<csr-id-84bd9a90b13048258607a138f91007107255d08a/>
<csr-id-7d719b3bbce07edd7c244d5ae904351d94306ee2/>
<csr-id-06c4ebd62862c3a1c3653dfea2c2d7c7e3b29b5c/>
<csr-id-3f484a6a43ff1146d6337bfe7d65869e9dbea5e1/>
<csr-id-3d9f793f4d0d1ba204eed0d8f44db7ab9c8b9193/>
<csr-id-89026500cbca3d512052705202f78151e37db19a/>
<csr-id-9f8034d489d2c9c8812bced71221f2254a32f919/>
<csr-id-f1c54563d198016e4d67bccb3de367f79f93ac3a/>
<csr-id-4eec949ee2ced73efd8dc0f642663969619dfd37/>
<csr-id-d13efc0565015b32149cd4dadd41bcfa7183ee06/>
<csr-id-5ddbe2f19f980eb2cd8d012f5dd4c7a86155a913/>

### New Features

 - <csr-id-4d0ae1cf8dfa71fdebd6bec6619b86ea22927b05/> Animation asset loader
 - <csr-id-78db6ac4abc0f078904085f4fed5a61e3e4ffac2/> Skip frames when delta time is greater than frame duration
 - <csr-id-98cedb030279d1188d751fc4595c02fbab5acab6/> Panic if animation frame duration is zero

### Documentation

 - <csr-id-e6dff14625bfa06edc2b83f0ef025b5cbaa98567/> Document how to load animation from file

### Chore

 - <csr-id-d13efc0565015b32149cd4dadd41bcfa7183ee06/> Require rust 1.59 or newer

## v3.0.1 (2022-05-18)

<csr-id-f6567790b015c9e815f0f093bc0706dd95be87a8/>
<csr-id-e9d9994f2844deb74647a31a853918b46f5d6738/>
<csr-id-005e48b22d0c4f023b8123696f30b3bae1dde216/>
<csr-id-731e37ef1e32fd9b0ed726533bc5be663eb9b638/>
<csr-id-6b0301091c4eb0eaaf807b55046e3447c3a2036b/>
<csr-id-f616a20d2b47381b2d6911d628f1334f8b925eda/>
<csr-id-bfc719d9997a27591b079286ab7e4d3b036a437d/>
<csr-id-44a05717b8baa10095996b530aa0577b626c9742/>
<csr-id-ed316dbe522189bed8ee14aceac49635f0ba7944/>
<csr-id-76ae99ce7395b00899100366cd11cb3e9c699ec0/>
<csr-id-fe9e0564a4835f6e6861107fb3b3d775558bda6e/>
<csr-id-329aec57b075d4b51222dfb9dc14478e59f3084e/>
<csr-id-1afd7160c89c8c1db022e24d48d69377a8554beb/>

### Documentation

 - <csr-id-feecaf780a6693c3520a042709d68c9b06cb1bf4/> Update installation instruction [skip release]
 - <csr-id-89e02e8c283d0c047b7f397d9e6d8bd2ef974391/> Hide deprecated members
 - <csr-id-dfa194683686ce5156e4c882e4dfd386197be68d/> Add ping-pong to the list of animation modes
 - <csr-id-1b00af46717612099bb64540f2aa00e0d89adb4a/> Remove some unnecessary badges
 - <csr-id-0164227f9449b82d7088d155cb5a225d943c1291/> Add MSRV to readme

## v3.0.0 (2022-04-15)

<csr-id-68acd0113e17f1e9f8baf6af6b7abd773a2bcf96/>
<csr-id-4403388154881dbc17629aa23bf884577e573f84/>

### Chore (BREAKING)

 - <csr-id-4403388154881dbc17629aa23bf884577e573f84/> Require bevy 0.7

## v2.2.0 (2022-03-06)

<csr-id-8e7b61827794ed7501dfdb0dc8ca6f708301b003/>
<csr-id-8fc688d88e57660405f5c71e96760672aadbc9a3/>
<csr-id-730b477423ffc9bfcf4b5e6c8d4f8125846ec79a/>
<csr-id-a5e7ea8e24bd12854908f48324415841f992ca1f/>

### New Features

 - <csr-id-1e306136a78baddbb2ae6fec660a4684acd851c5/> add `current_frame_index` getter from animation state

### Documentation

 - <csr-id-91f9aba814fb8af0ce1210e6a25c2b3eb597615e/> update version

## v2.1.0 (2022-03-02)

<csr-id-f06ad5bc39c8168e359d2a84d9a264c665d930bd/>
<csr-id-e9c4de6654bab0455cd52ba509ce2342380eaa53/>
<csr-id-db4a130ae022ed9b91d16e21268bc324f274a6cc/>
<csr-id-b00f9f539e08f6072cc56cc4e9d285ea5a3ce214/>
<csr-id-362956e26efc135e2d8e2c64433c1a42420bcfd3/>
<csr-id-ace3488a44457f596ec6f3467d12eb92768d6144/>
<csr-id-79ee97ca171dfb32f8f1ee27244e4bbe863cd0de/>
<csr-id-48b917846c32eacd6adf8e81e28d88c2af2f321c/>
<csr-id-c381d6f5326afc87bc682fb4ab236b987b42b18c/>

### New Features

 - <csr-id-586859c06cc3a63081d0f13156927c3a1db2bc0e/> Add repeat-from animation mode

## v2.0.1 (2022-01-29)

<csr-id-b611b61812c3341b629288d7e8d19946e98d2e5c/>
<csr-id-517cc9ba42f5bf1b170f27e68dc3812fc0aac01b/>
<csr-id-8fe60eb4704591f891bb422f988171238a45afe8/>
<csr-id-43ff85029dc451fd0c6335c13271ade90cf67a6b/>

### Performance

 - <csr-id-7f37562e19b3ff48388f2f359ce562ae153fb0d5/> Faster insertion/removal of the `Play` component by using `SparseStorage`

### Documentation

 - <csr-id-bbc666d43d592d1a00acc037ff21f7ade102852a/> Remove reference to semver

## v2.0.0 (2022-01-24)

<csr-id-8e7794cd179fd6c36d76af1265ea23ed72cd5090/>
<csr-id-0ac9a1576b5e35736198a0ce0f5ae9e61c57b74c/>
<csr-id-58c8e5298791d6adf281599f7aeb3fc1f7b383aa/>
<csr-id-74c3ea02853da5d58c28ca99731c93cb93e1c627/>
<csr-id-8acb6d24927f7fea7a79009001be07148149e0e9/>

### Chore (BREAKING)

 - <csr-id-8acb6d24927f7fea7a79009001be07148149e0e9/> Require rust 1.58

### Refactoring (BREAKING)

 - All struct fields are now private
 - All enums are marked with `#[non_exhaustive]`
 - The constructor of `AnimationPlugin` is now private
  Use `AnimationPlugin::default()` instead.

### New Features (BREAKING)

 - Add ping pong animation mode

### Documentation

 - <csr-id-5de0b4bba4447ef983f1b2ae0435d2349955b989/> Update bevy compatibility matrix

## v2.0.0-rc.1 (2022-01-19)

<csr-id-df66361b4ba601cd2c6dffa363a1dcfd936bdeb0/>
<csr-id-249fc6816f1984b85d7410a32d5f0b493a0216c6/>
<csr-id-b782ce97e19f54670d3f990de403042ecf4eaac4/>
<csr-id-8acb6d24927f7fea7a79009001be07148149e0e9/>

### Chore (BREAKING)

 - <csr-id-8acb6d24927f7fea7a79009001be07148149e0e9/> Require rust 1.58

### Refactoring (BREAKING)

 - All struct fields are now private
 - All enums are marked with `#[non_exhaustive]`
 - The constructor of `AnimationPlugin` is now private
  Use `AnimationPlugin::default()` instead.

### New Features (BREAKING)

 - <csr-id-76a6306c6becb3f1ea6c1bbfabf36cb8bd9e2de8/> add ping pong animation mode

## v1.1.0 (2022-01-18)

<csr-id-1ca58ae6f1456791cb6e06335f59544b329ae82b/>
<csr-id-9c0276821f2e72ed5337f6391b8d38cd00814783/>

### New Features

 - <csr-id-6e670db5f162a963318fab2759cdb4a5f3fd18b0/> Create animation from iterator

### Documentation

 - <csr-id-6be327fbeff7bb1234e0214c436f2faffd2b873b/> Improve format of the bevy compatibility matrix
 - <csr-id-2bc89f71a07064c11b793ac2de7818623101fe95/> Fix bevy compatibility matrix
 - <csr-id-9da0890d37f4046ec28e117f8bce6372193976c3/> Remove obsolete recommendation

## v1.0.0 (2022-01-08)

<csr-id-0a642c5eff46e8e6b1fd50c4d4c3200a5d5ee776/>
<csr-id-731910b61409a7fd9713872bdad043da148dfef7/>
<csr-id-3e989670925d541a8c42ba40868a9bd286cbcd83/>
<csr-id-1c4bfc331f6baa8db193d259389c0ac2846fd680/>
<csr-id-9c32d8c2da7cdfb73528f12d8f4ec3dc8781b2fa/>
<csr-id-dbed43c681d1b10d96335ee8e65f708fdb06bc79/>
<csr-id-a4b9548fa2d670877584f732b33b26d73b7d7b0b/>
<csr-id-a05e98a611e8d618b85dd8b14a9d384d8c346d61/>
<csr-id-9f4b3cf502bfd8f7565076e5baffe54673b9a692/>
<csr-id-937b45ba08ee52c489eff275a3ecbe11ce5c9516/>
<csr-id-84fa52b980829906b26206a53263ff183cd619eb/>
<csr-id-f9d932a2d456321d4377d0fca64e36a0e568cafa/>
<csr-id-76d12564e3a82e4fd3f82652df180c41d68a0e08/>
<csr-id-7e08b25974e6b003491429f0e93f0a69fd4a3e77/>
<csr-id-b308af22f62f1beb53ff431b22f8b0d6059b7ae6/>
<csr-id-bfeb347c86ac2a300405b304b1f564fd40c4339a/>
<csr-id-3a38e80030ed436607f8b4118700eb4a009b5875/>
<csr-id-9b951b662579c6e88c3a330dfc4f96cce8e4263a/>
<csr-id-110819092e0560f81d3ab279547d74f839c0e872/>
<csr-id-d68c7a81decb19d8c7e07fe04f704e31eed99c78/>
<csr-id-d19b17d5033383a5ec0173111b325c8a3c9b0645/>
<csr-id-da80628b01bcde4eb0903908797d5aa2318c839c/>
<csr-id-f88f9152f49d0d55bec3650e85b6517b826145e5/>
<csr-id-c78c89ca83422633307351bb6d5af8dfd02062f1/>
<csr-id-73fdd38aa054937f9cfa59b1eb3ce4dcb1f1c904/>
<csr-id-99da92b094f6ba855e6ce1de592ed483aa2c7064/>
<csr-id-04af5e1e8b45ad5717f2798880dfb6de85d8af0d/>
<csr-id-9f7f6874046272ffa8dc12ada5176983d7fcf3d5/>
<csr-id-54583e06adafdf500b3443e973b588a9283a6d62/>
<csr-id-1ebf47e3b746cfb1adb8fa33bed42028c780c692/>

### Documentation

 - <csr-id-439c37d86fe0916443b3768b98044741647d8ef9/> Minor updates after upgrade to bevy 0.6
 - <csr-id-d77209b6c1986de194f202c48e99c6645a3420ee/> Clarify that SpriteSheetAnimation is a asset (and not a component)
 - <csr-id-f2e6daccfc50479ff8e9525a744054ad0f37481a/> Remove (broken) audit badge

### Chore (BREAKING)

 - <csr-id-dbed43c681d1b10d96335ee8e65f708fdb06bc79/> Require rust version 1.57
 - <csr-id-99da92b094f6ba855e6ce1de592ed483aa2c7064/> require bevy version 0.6
 - <csr-id-04af5e1e8b45ad5717f2798880dfb6de85d8af0d/> Remove `Reflect` implementation from `SpriteSheetAnimation`, `AnimationMode` and `Frame`
   They are no longer used in components. Reflect unnecessary leak some
   implementation detail that cannot be changed without introducing a
   breaking change. And they caused a problem while upgrading to bevy 0.6.

### New Features (BREAKING)

 - <csr-id-2bcee87fee72460755af1ff562838e431d8d0cb9/> Update animation during the `CoreStage::Update` stage
   So that we can detect end-of-animation during the post-update stage.

## v0.3.1 (2021-08-02)

<csr-id-194e991951d77d6fbdfd5c3dba369cc118c3b450/>

### New Features

 - <csr-id-1d547900cb9879e9cc678ab1573e557923f0d848/> Allow to reset animation

### Bug Fixes

 - <csr-id-4c7b5ad6c008d9893be90c4ae366f9afbe8006ff/> Impossibility to restart an animation ran with 'Once' mode

## v0.3.0 (2021-07-19)

<csr-id-34d6a83eef0358c17d9ae05e7fb0b4a7066cee27/>
<csr-id-f048404a709e13b3f6ca487ebee49dc448d0d5c1/>

### New Features (BREAKING)

 - <csr-id-2a895a5417f9ce60efc81449f12868d5cf73883b/> The SpriteSheetAnimation is now an asset

## v0.2.0 (2021-07-12)

<csr-id-8b2fb369b479d6e9c325ffd96be8ef0bce52b386/>
<csr-id-f300e640fa9e4218c00e2db8cae3b437ec7be5dc/>
<csr-id-7d9c9aca7cce35300325d8b919d89c56fad7dac6/>

### Documentation

 - <csr-id-00164ffeb2944fdbcf7cd80b64c767c1b7a3941f/> Add change animation example
 - <csr-id-edc6936f88220d360a97a5a9e8a6b515d703b8a5/> Update contact section
 - <csr-id-d32180f410aca1e463c893888825407e625495cb/> Credit author of the coin asset

### Refactor (BREAKING)

 - <csr-id-7d9c9aca7cce35300325d8b919d89c56fad7dac6/> Extract animation state into a dedicated component
   That will make possible to move the animation definition in the assets.
   
   And, it also makes easier (for benimator and the user) to react on
   change of the animation component, since it will no longer be updated just because
   the state has changed.

## v0.1.1 (2021-06-16)

<csr-id-b548a7a21c10683654c530266c79e652377abaf5/>
<csr-id-1e3ddb164810d8d944095d753d2a4cb07de0c83c/>

### Documentation

 - <csr-id-1e3ddb164810d8d944095d753d2a4cb07de0c83c/> Fix project title in readme

## v0.1.0 (2021-06-16)

<csr-id-440cb268e9c8e0a8b9da33315979db1c30c5eeaf/>
<csr-id-363765cdb5506b7fe37051dee7f530200d39d725/>
<csr-id-2c1b4bee94d86142ddabb2700cf05d94b3817032/>
<csr-id-e4b4ef86cfd0598d0853e3bcd5855b162feb2bfc/>
<csr-id-d79df6620f1fe16a3fba20ca98fa856559c3a4e5/>
<csr-id-694409cfbe7cefb5dd5708d9768902fec1676e79/>
<csr-id-27bccf4e92bb43974c7a248b3b3669c0d62ab4a4/>

### New Features

 - <csr-id-0efff6bf5ded1e83505c1bdcad05122588d8e296/> Run animation once or repeated
 - <csr-id-46a03202781c6fe45bb73a10da31678ca1dc751c/> Create animaion from index range
 - <csr-id-e13a69c7923279f5e26e761426ea62d797f4d4b3/> sprite-sheet animation

