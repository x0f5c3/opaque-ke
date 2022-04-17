// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under both the MIT license found in the
// LICENSE-MIT file in the root directory of this source tree and the Apache
// License, Version 2.0 found in the LICENSE-APACHE file in the root directory
// of this source tree.

//! The OPAQUE test vectors taken from https://github.com/cfrg/draft-irtf-cfrg-opaque/blob/master/draft-irtf-cfrg-opaque.md,
//! which are presented in https://www.ietf.org/archive/id/draft-irtf-cfrg-opaque-08.txt

pub(crate) static VECTORS: &str = r#"
## Real Test Vectors {#real-vectors}

### OPAQUE-3DH Real Test Vector 1

#### Configuration

~~~
OPRF: 0001
Hash: SHA512
KSF: Identity
KDF: HKDF-SHA512
MAC: HMAC-SHA512
Group: ristretto255
Context: 4f50415155452d504f43
Nh: 64
Npk: 32
Nsk: 32
Nm: 64
Nx: 64
Nok: 32
~~~

#### Input Values

~~~
oprf_seed: 2ed630416cb2e532804133133e7ee6836c8515752e24bb44d323fef4ea
d34cde967798f2e9784f69d233b1a6da7add58b2c95a57bc213aca920c14553ed2d83
3
credential_identifier: 31323334
password: 436f7272656374486f72736542617474657279537461706c65
envelope_nonce: 36168448f9c5ec75a8cd571370add249e99cb8a8c43f6ef05610a
c6e354642bf
masking_nonce: 13573601f2e727c90ecc19d448cf3145a662e0065f157ba524df0d
3e56ad6236
server_private_key: 51da1f6c3ea07fa00c7cbfdc1fdc70659f1a1092402da749d
938c1a6a570f103
server_public_key: 583f7bccccbc1907ae1506bac950d08266eb3b33ba452b8df7
061a390ffd736e
server_nonce: a88904fe660061c4fac7e452066b8b0f90da7d8d4a19f1cc41fb6fa
5479b467d
client_nonce: 400ceac0fbfb16005928335518be6f930a113c6c0814521262e17ec
c3cdc9f91
server_keyshare: 5cc9fd06a5917ab66a6ef5537a65525a428f768840d81a00d82a
23fc5491b53c
client_keyshare: da25553da9ac142b36332dbd487713ae6712432fb317a6e00b2b
17525bbe6912
server_private_keyshare: eb7216a0ad73af2e84aeeeeb39a9e3549f0817e1732b
5faffc5e0f5abf269e08
client_private_keyshare: a2582d86bf4476a413caa6ee0d3daf7fb6908909036e
1423170d0072aad0d00f
blind_registration: f349de058878adaf864afedb28cf6a6b0f7083a11c34f9543
9c5cb44edb7fd09
blind_login: 146538c20e42b5182766e71c26d4e3a4d1b9c493f7c94bea0bb4f9d6
31181c08
~~~

#### Intermediate Values

~~~
client_public_key: eabdf39b4f22d045f80477d5571bae4c40e13377bcb410c6d8
6d86eab281eb15
auth_key: ca657130e970f04883cb6e1d25414c2e6b790521d2589eedbb28f88c2a0
cd1d47a451af444604838acc7ed0eb06cd15265a8f2008f6c00a01471c30d0dce45e0
randomized_pwd: 46dac5eed750784bf22be60303312d53fd6ec61cc19bec55c136c
3629366b1916e8e6a1b09ad9e079da2aa9ce0cde3aea3f28d835b2f67c8bf6e5139e5
e3cc03
envelope: 36168448f9c5ec75a8cd571370add249e99cb8a8c43f6ef05610ac6e354
642bf72e2cdbb55ab0d0cdfcf1cfb9344d3ccbfbcb1b69f975e2e58f25749214ddb7a
11ec03ed7d3f04f05c0c822bd2a4d6cd61c7911035ce117e34f6bc4d8d27ba95
handshake_secret: 71d30b66205d8f3d35415facfa654c45c778ccb1a1522b0cc38
fe88f0eba0e47e4ffbd13cee0bdf0b4cf4b97fb50417bb799d4cfeb58471abc2302dd
264dbe9d
server_mac_key: 6ce3829a4eb1758bbbb9263da5c989b6060851fcae76d10af1a1a
17a627121cc327ac65add4a93d1f3fb289d4b741481dbcbda570a03d156a0c805e287
487db8
client_mac_key: ce946912e6fa49c11068184bdfb0c1d7cb0bb69d2d4ba15dcdc28
bba18021850d296c5c72fa68848ea4c8927d28065c4807fc8163275f0781bffccca18
72ba31
oprf_key: e4af4ba0d3e3d3340848000b77ab12e736fb1662ffbe529ec92163d37ae
26601
~~~

#### Output Values

~~~
registration_request: ba1a2238a29a33dea928801e0257bd644f34bcc12f3e6ed
eba3a5015b45d6e33
registration_response: 1c5078bb63f7623d65926a6ef82a4ee7d1b62225d5f8a3
59f603475654f4453b583f7bccccbc1907ae1506bac950d08266eb3b33ba452b8df70
61a390ffd736e
registration_upload: eabdf39b4f22d045f80477d5571bae4c40e13377bcb410c6
d86d86eab281eb15ae21afa59b900243876169f04c46a5833b8168cd87ce9e5a5c04b
ea74eb523bdbeab479e62632bb24f6e4a16fa3ae2132fcd2d4ffcb5cafce1cc8394a8
c3eb3436168448f9c5ec75a8cd571370add249e99cb8a8c43f6ef05610ac6e354642b
f72e2cdbb55ab0d0cdfcf1cfb9344d3ccbfbcb1b69f975e2e58f25749214ddb7a11ec
03ed7d3f04f05c0c822bd2a4d6cd61c7911035ce117e34f6bc4d8d27ba95
KE1: c021ab3bca8c7c7949f7090d2af149523c5029d6c5c45b59997f8c306ccbdf75
400ceac0fbfb16005928335518be6f930a113c6c0814521262e17ecc3cdc9f91da255
53da9ac142b36332dbd487713ae6712432fb317a6e00b2b17525bbe6912
KE2: 1aaae8c352e89557d73dd57152f10983ba4871675d5307c71fc8f8d808103707
13573601f2e727c90ecc19d448cf3145a662e0065f157ba524df0d3e56ad62366311e
350706148302b24efbafa041792c5b79e78c43aa24b44c6e81dde926692d9a9095273
212a862729bad5a9e5258e7f1bf656045dc2842d331d183cc7425c1953a5cd8dd3b4e
83638980d0a85a2c2204eb8d3879421a43450d7eed4bd203b99e16526b8933fc46a62
4a1fec3caf6a5eebe2dfe9689b847716b330098638d1a88904fe660061c4fac7e4520
66b8b0f90da7d8d4a19f1cc41fb6fa5479b467d5cc9fd06a5917ab66a6ef5537a6552
5a428f768840d81a00d82a23fc5491b53ca3e00703736229ba774fb92ba77dc2a2236
e408b99cd8b8e2b0fa2a92ac132100f807b3e44ff1c60d3939ac6b6e8719a4ddf2b37
83f4650fce842ea5c63ccc19
KE3: c5cceaabc721066f2332edcc8cb70c49b8930639f31c6f3ebd8b9e232d35462e
a00bb0bcfa0b703d8b20f06d3428ee7089c299829b737f42a32a26519e33e2bb
export_key: 421f6315d5a2dd7d17eb13c596e69a4455b99209264be00181e99dedf
f76d5a5f55e9cc1340a078f8b307c9dcd95d391193b1ebf648c98378871d087620a0b
a2
session_key: fc56461df9021851b65b29169b0666e3af085c217079db4fe4881073
d9796a2a9add0878ec647f841d2e6d8aecb4d3df8fbc13970a1647b743d29fc5cc892
dab
~~~

### OPAQUE-3DH Real Test Vector 2

#### Configuration

~~~
OPRF: 0001
Hash: SHA512
KSF: Identity
KDF: HKDF-SHA512
MAC: HMAC-SHA512
Group: ristretto255
Context: 4f50415155452d504f43
Nh: 64
Npk: 32
Nsk: 32
Nm: 64
Nx: 64
Nok: 32
~~~

#### Input Values

~~~
client_identity: 616c696365
server_identity: 626f62
oprf_seed: 4f8c9a5c6576fe6cb958f149fec78f4d8a2875bb40615f6f44ecc2fe30
635396b708ddb7fc10fb73c4e3a9258cd9c3f6f761b2c227853b5def228c850fdbf1e
2
credential_identifier: 31323334
password: 436f7272656374486f72736542617474657279537461706c65
envelope_nonce: 39886c5188df91d7e03ab3f513b828850a017408ffdf4fe072d40
d012f55f6ac
masking_nonce: 983deeb54c9c6337fdd9e120de85343dc7887f00248f1acacc4a83
19d50e29b5
server_private_key: 7f02b3727a18c1d885605e9e09482e22555110f5d2f31a63f
7f8c17f6a985d0b
server_public_key: c0c1fba5133d9b9b5055287de8c8dea9dfbebe10d12ebdf4bd
8ed249886cc67e
server_nonce: c6d04efaee8370c45fa1dfad70201edd140cec8ed6c73b5fcd15c47
7408184fa
client_nonce: e8f5bbbaa7ad3dce15eb299eb2a5b34875ff421b1d63d7a2cfd9096
1b35150da
server_keyshare: 2c3dd46ee4b31250f28ead72fe3d8268ef89d25c9c6318189b9d
04cc729abe51
client_keyshare: 8824e44af3cdc7a29880ff532751b7ccc6a8875ac14e08964942
473de9484f7b
server_private_keyshare: 7f5fbe5a989043f533b588f3c89b21c9dc7991b89ddd
28cde4be79afdb83170f
client_private_keyshare: 9909ef87bfd10d3148a64f98e619251074345b023f19
931b1652c9934a933104
blind_registration: 45075e8ec6743c394e85e3f81ce383ddf78791d163b457fbe
c78c58c0a55050b
blind_login: 6a7637875c6c59544c262523812302dbec1fc73a01abcdbeadfe898e
54dcfe05
~~~

#### Intermediate Values

~~~
client_public_key: ee597ed63b18ccc6e5b77ae703e3bd4cfd574650284b21c64e
c16926da7e2851
auth_key: 0fa176059cf53854c38c9841f8c5d5a756b297528729b4a4b5b7894eec9
7b1d9dacb29c337a48cbc276db45452adaeb77e1b4f4990b8d0ef4c03413d3af4a274
randomized_pwd: 15490043fad1f612a0cd72f7571f720f2e5bff138b6c0f9a3f8c7
feb028761ca6bb602028e4228cbc1bcd9b1a8dc3500a7701d9351864595a765ba6a4c
c1b2f3
envelope: 39886c5188df91d7e03ab3f513b828850a017408ffdf4fe072d40d012f5
5f6acef003030d52697e8bfd717c1db8c5ff7a2c0112d7484f1a567c942612c718b5f
010978c806fdaf6892c7ec16d50f80fa12e33daf798e96a71064c72942478bc6
handshake_secret: 540feab4c07eb9263b828c4c20a6138adb46541de1633da67ce
1393a03c1f5e04167a8b0336b45e8aa2a1c3d8c9452f9aed7b0d54545adfcaaa0aaca
35b0a573
server_mac_key: d8775d094511b77e17f4433d6cbc53f4b34b69db34a16c8feeffe
573f70175fb0e16f61ab8ccfddf3599f46ccaa95898b8cefc24c3e73d8a900ea4f0c6
bdbf86
client_mac_key: b7e1601a00b647a559a25a2f30eec8f1105ff51dbebcaa506d943
ec2032c0d85c07673c3784c1008493b8a794a1cd2ef8d4972e9472a665c3abee685f2
03f629
oprf_key: 671cd2624e173c4df9ff81295c41007bf64fe10dec3cf9fd90365040ba6
e290c
~~~

#### Output Values

~~~
registration_request: 3e054b6596da6f0da124baa2c095a29c3a6b48571aae699
96f0e079067ac4172
registration_response: c45804cfaa87737d2309164bf7fc0567358c9fef629afd
47a17440d7d43ee71ac0c1fba5133d9b9b5055287de8c8dea9dfbebe10d12ebdf4bd8
ed249886cc67e
registration_upload: ee597ed63b18ccc6e5b77ae703e3bd4cfd574650284b21c6
4ec16926da7e2851dffc6e3207fd7ad90fb974e8f35d17a1f60c0fc9e6cbc49375917
556413a1dac4f9c719e6f63055055276c46d5a308dd4c3f07ca3061176a7ef9200b9c
4a451239886c5188df91d7e03ab3f513b828850a017408ffdf4fe072d40d012f55f6a
cef003030d52697e8bfd717c1db8c5ff7a2c0112d7484f1a567c942612c718b5f0109
78c806fdaf6892c7ec16d50f80fa12e33daf798e96a71064c72942478bc6
KE1: 7002a52fa6c2916c49c1fff952e818e458c7f7799139b243918c97758f463a47
e8f5bbbaa7ad3dce15eb299eb2a5b34875ff421b1d63d7a2cfd90961b35150da8824e
44af3cdc7a29880ff532751b7ccc6a8875ac14e08964942473de9484f7b
KE2: 6e78c98f76160c8cb4df1d0cf3fa038a32b900a1f208901b69b7fb695c28001d
983deeb54c9c6337fdd9e120de85343dc7887f00248f1acacc4a8319d50e29b5f9f61
75f37e8a0718398038dd5159f049f6e7f96be9754907827de30738109889169846cea
a7eee3a6109334a84fd6ec3bb5d462d5b87359f1d909ca5e9b0e7b43000dba44fb4df
9f1629bbe20dd92de2972072ac4ddae968c2dadba8614afa8f0f29cd67ba8e18ced81
49290e67f772f4ff6984a1fd4f163dc2325841eb723bc6d04efaee8370c45fa1dfad7
0201edd140cec8ed6c73b5fcd15c477408184fa2c3dd46ee4b31250f28ead72fe3d82
68ef89d25c9c6318189b9d04cc729abe513ce37681b1db692d3f47e486c31c22e4390
95dc9a4155dca22a5d2e6b8a517f2f7a5d8cb0df01673030683f72a0f62bb0941350c
68d9dc7c449aaa0140bba686
KE3: 4f74844e0c86abbc9189cb03f57e807e2034bdd07f17e67233010a6cacd9ef11
0f153418cafca68e0f8f4f48234d705089f64a7b47bacd0abea3f2a574da5629
export_key: 5c54270bf510936861ea01444d70a7204a6fe1de33ca9613d41e02d30
0d1e6a90b15cabee67a0129629f6b3aac173e1483dfc43457d72fe6df6524a639f89a
1f
session_key: 391db76593cd7f7766b68de34f99b8c0253e86914dbb18177c011d3e
05d611a3a2d0ef7a2b58468c1549444f81a60afbf635d2f6f878fc63061ecc94cfb27
ba8
~~~

### OPAQUE-3DH Real Test Vector 3

#### Configuration

~~~
OPRF: 0003
Hash: SHA256
KSF: Identity
KDF: HKDF-SHA256
MAC: HMAC-SHA256
Group: P256_XMD:SHA-256_SSWU_RO_
Context: 4f50415155452d504f43
Nh: 32
Npk: 33
Nsk: 32
Nm: 32
Nx: 32
Nok: 32
~~~

#### Input Values

~~~
oprf_seed: 380d78c283bf98e26334038293e47865922a3b54d3722d8e9ced1c8729
c42f5a
credential_identifier: 31323334
password: 436f7272656374486f72736542617474657279537461706c65
envelope_nonce: a994c5c01c1855151c467aa331d70f59d9bb63e9afa1e314672a9
c7c6e460d5d
masking_nonce: 848bdf20ed725f0fa3b58e7d8f3eab2a0aace261f61193c7f85709
e9794357fb
server_private_key: 63b448daf85853343c35ec32253326810d0d88f0936c712d3
e901b42cb792f37
server_public_key: 02217c73e50ebf9f8ea0e080a2ecbaf594ca7d5828984e8d5d
455d42ac8531e4f1
server_nonce: 84ff1f2a310fe428d9de5819bf63b3942dbe09f991ca0cf545e33a8
fa17ab9c6
client_nonce: 72721898ef81cc0a76a0b5508f2f7bb817e86f1dd05ca013190a686
02c7af25f
server_keyshare: 0212d788fc5776bd88b7aa01e72ad0d147d8c8a3d9e47d94ca79
10e29f11297b34
client_keyshare: 03a51c7c3d3a69f5217c0f8de4efa242b0cf4ba35cc67c820e57
b69e7a4f53cd69
server_private_keyshare: b3c02a66ef9a72d48cca6c1f9afc1fedea22567b0868
140b482123652ea37c7f
client_private_keyshare: 5d25f85613f5838cd7c6b1697f27bb5e8018e88ecfc5
3891529278c47239f8ff
blind_registration: 7b5d31d5e3ebdb127f92416a3cbcda76e24b2be8d08c79074
a5520292916911b
blind_login: 47401b35db40bdc28cd90b502b3390d3cfea5814c105ca7b460cdf8a
7012c76d
~~~

#### Intermediate Values

~~~
client_public_key: 030068ab6e722bb6593382a86becf60ed8290650402470c21d
c90bd0ea9da0f19b
auth_key: 8813ac116d6d46df161221d53ba5ec3bd68baca857c9ee8e3eecb7fc162
08fe0
randomized_pwd: f19aa5337d0ef8c7f728787df75f9abb6a0d06c854960d0646844
c8d68dcc3f2
envelope: a994c5c01c1855151c467aa331d70f59d9bb63e9afa1e314672a9c7c6e4
60d5d6465561f86591334921a1c4402ecbfab336a9945ce398848eff0990b44f4b6a0
handshake_secret: 019dee3711ac01beb7674207a7ed2814f67658d10c52cd71d71
6b87e4204d9a0
server_mac_key: 922a759956ae32addb64e55343677c08538eeddcba9a8b4e861f2
1e9c3849d5c
client_mac_key: ba64564e701ed14b35c6c0d124f6dc98ee1a138c40b4267079363
7419654cd28
oprf_key: 7ad47c7aa69dc3700c91449472d4bd09b15543683560870c7dd21b78398
0f7bf
~~~

#### Output Values

~~~
registration_request: 0347ed9a28ccf8baae3b312837378fbd4f994bf601a2522
0bc404102bd1cd9e4a0
registration_response: 027818306df41ac75916146c9d0f06d842e83f232a61da
40b660ee5d670cf77b8202217c73e50ebf9f8ea0e080a2ecbaf594ca7d5828984e8d5
d455d42ac8531e4f1
registration_upload: 030068ab6e722bb6593382a86becf60ed8290650402470c2
1dc90bd0ea9da0f19b4344705e052a843c4cced8ce7c87478555cff1323fc64063301
9423a19455e53a994c5c01c1855151c467aa331d70f59d9bb63e9afa1e314672a9c7c
6e460d5d6465561f86591334921a1c4402ecbfab336a9945ce398848eff0990b44f4b
6a0
KE1: 0226bc3aeccce9c813eaec852599fe76eafe611467a054e738441d4a3b7922aa
ba72721898ef81cc0a76a0b5508f2f7bb817e86f1dd05ca013190a68602c7af25f03a
51c7c3d3a69f5217c0f8de4efa242b0cf4ba35cc67c820e57b69e7a4f53cd69
KE2: 02745cdd4d8336647d5de1715fff6a639b8799e3c6ad951faae59203f4bd97b7
89848bdf20ed725f0fa3b58e7d8f3eab2a0aace261f61193c7f85709e9794357fbd6b
dad9096bf4fa824a2e78e8f36209c9a7fbac3ccd0d56c2b6ea9a0cca3ec7691594eaa
bafbcb4b8b32b65dd8e9fe7e903d9639d67787a2ef7d88d06d257f791eaa59fb7a8b3
d8ec4186c6707b2942dc6ef990e8b958d79c27587f73d371a7cc884ff1f2a310fe428
d9de5819bf63b3942dbe09f991ca0cf545e33a8fa17ab9c60212d788fc5776bd88b7a
a01e72ad0d147d8c8a3d9e47d94ca7910e29f11297b344439d99f9408b8047da08d4d
6ea017e571a26a9a1d80440ed9e4793684dd463d
KE3: a453c142682a3247cea48735543911b07c7498c1c3a7908b8b60c8e1fb90adf1
export_key: feda4a04aa974c1ef9c9d047eb2909ee175851f1c0f5ba37929673f0e
46235e4
session_key: 3585c6e3365b8ad1daa5fd7c3878de2930e6d844bdc8fb13f09debce
b82fde22
~~~

### OPAQUE-3DH Real Test Vector 4

#### Configuration

~~~
OPRF: 0003
Hash: SHA256
KSF: Identity
KDF: HKDF-SHA256
MAC: HMAC-SHA256
Group: P256_XMD:SHA-256_SSWU_RO_
Context: 4f50415155452d504f43
Nh: 32
Npk: 33
Nsk: 32
Nm: 32
Nx: 32
Nok: 32
~~~

#### Input Values

~~~
client_identity: 616c696365
server_identity: 626f62
oprf_seed: b19c2b0ccd8ba22218b6c772e19c4174dc8f436b55b69a4fd701d69873
dacfeb
credential_identifier: 31323334
password: 436f7272656374486f72736542617474657279537461706c65
envelope_nonce: 3f1640a6645455ac63788ee075c245690f9669a9af5699e8b23d6
d1fa9e697ae
masking_nonce: f1029631944beed3594c283c581ac468101aee528cc6b69daac7a9
0de8837d49
server_private_key: 31ae68b478bfc59f5ef534d4e0092e8ef1bfe338aaa4b65c0
563d42fe20626a0
server_public_key: 025cbe5babe2fb2b94ee2527bcdc66fd3a62f4b7e724bdb3ef
4a41cfee527434f3
server_nonce: df174426b40de97e2fabc448b1f4ab66a1a3149df447696d2838463
8319c3819
client_nonce: a2912bab9b6a62cddf7d5e3209a2859e5947586f69259e0708bdfab
794f689ee
server_keyshare: 02ad94bd9a2bb46d8e8ea26ae480a24e2825f58560a20d583a3c
c5078849bdfb8b
client_keyshare: 038744dec9da18441e1ef78ff9b2e5d62c713e56eee7aa326a9b
e577365f919d6c
server_private_keyshare: 711c04899739c0620dc94323d026011ac6def373c257
5400d4018ae26bb2437c
client_private_keyshare: 708e76310767cbe4af18594dfcd436216c2658300d05
18d56d002be476bd06c8
blind_registration: b4526267d942b842e4426e429d05ea84aa6ab34552f0c4a3b
9efdcacbf50daec
blind_login: 12176d4f7ab74fa5fadace604308682dc1bdab92ff91bb1a5fc5bc08
4223fe44
~~~

#### Intermediate Values

~~~
client_public_key: 02bef9b16c148b03218e5f8b01a4b52d5cea4a51ddbc76743a
13ba2fa5d1631b33
auth_key: 6dd41a206a9f6a75e02e80e7bb4e696ee2ba68e01e1f96c65e1afc9556a
e1ec8
randomized_pwd: 95847555e29a90ecd2af4e26343be5c65e1c347f1d921be48ba9d
f4e61fad23d
envelope: 3f1640a6645455ac63788ee075c245690f9669a9af5699e8b23d6d1fa9e
697aebc8f31f964da8a9c11a21b359f7522ae50bc02c85362e7dbf051bdf3bf113d98
handshake_secret: 19b7cd9abb29842a0786aec00a574d29f6080cf128840c4867e
2077ba430b621
server_mac_key: 700c2b8c1797a44829e511aef4c66a49ea43aa5ef2847e4993946
798d8d7cbdf
client_mac_key: dfc3af348aa8baf3e95eb904fdfdb11cf8606b961f14dfd5d6881
7658b4d7178
oprf_key: 0a663cda294cc97edada43ff06235a23ac53bf55c439ecc664c01e44738
74e65
~~~

#### Output Values

~~~
registration_request: 024cd26832a141c12564716b57b3101d281193c3a2cfaf4
b4d0217b98c69a6e356
registration_response: 034b85dfb783b81cfdcc2255b6ba440479439c17e5f566
690de0dff23ab08bb153025cbe5babe2fb2b94ee2527bcdc66fd3a62f4b7e724bdb3e
f4a41cfee527434f3
registration_upload: 02bef9b16c148b03218e5f8b01a4b52d5cea4a51ddbc7674
3a13ba2fa5d1631b33f8295762da035ff51f6ae4c07fac29e73b900f39a6be5a222d1
43e466282bff13f1640a6645455ac63788ee075c245690f9669a9af5699e8b23d6d1f
a9e697aebc8f31f964da8a9c11a21b359f7522ae50bc02c85362e7dbf051bdf3bf113
d98
KE1: 03ff69ee0b845955eafc817acf721fdecccc94977c4aa0841ec33bf5060375e3
a4a2912bab9b6a62cddf7d5e3209a2859e5947586f69259e0708bdfab794f689ee038
744dec9da18441e1ef78ff9b2e5d62c713e56eee7aa326a9be577365f919d6c
KE2: 026c5dfb5840f9e18b49a2553083bf600b23d73a5352a289223d2a1175d36c9f
b0f1029631944beed3594c283c581ac468101aee528cc6b69daac7a90de8837d49b76
72e17bcd95b17b4d599321921a0ff1d3783624edb14480c018bf39e7a7bab84752a79
7ced451076a5542cf5b0b8433d2b8cd5ceaf9ef7c5f9c1ac13a3e9ff6242362de7710
c5106109ea6a6889388a62a44c932e225c18d649bb44df09b0fffdf174426b40de97e
2fabc448b1f4ab66a1a3149df447696d28384638319c381902ad94bd9a2bb46d8e8ea
26ae480a24e2825f58560a20d583a3cc5078849bdfb8b3d44038b2e740b28519d83f3
8b58cfc221a4421bca2eb74efd05f5c31b34190b
KE3: 6ad96f7b8b167c3482babae788482ddd2ef417eff9ad5617ae49f6c35613b723
export_key: 7284190bd6a6e175cf38846f1374b5f81a481200f774482d89bdb93e0
3674f15
session_key: 4114f3f9ddb7d6f84fc479ab1cbf2e6470540e814b75329661d22fc5
5a8eadff
~~~

## Fake Test Vectors {#fake-vectors}

### OPAQUE-3DH Fake Test Vector 1

#### Configuration

~~~
OPRF: 0001
Hash: SHA512
KSF: Identity
KDF: HKDF-SHA512
MAC: HMAC-SHA512
Group: ristretto255
Context: 4f50415155452d504f43
Nh: 64
Npk: 32
Nsk: 32
Nm: 64
Nx: 64
Nok: 32
~~~

#### Input Values

~~~
client_identity: 616c696365
server_identity: 626f62
oprf_seed: 28885f5b834484836667b5ffb0ecf900c07c55d70e9894af0231f52c54
dd29cccdae5fae5b60c92fa3cd7e6f042429c7c9e946f5351292fa08f4e99e395c30a
9
credential_identifier: 31323334
masking_nonce: 59d26775ae953b9552fdfbf2ab6f469f2f153f9a88aacb7ed434ae
d9fd7ac1ab
client_private_key: 11e4c3344def24f8f55f46f9b72584b36ce931e2a11299afc
6093dff0fbf470f
client_public_key: 020693a36a55d62c38bd2d5f1aaeac2a918e90e1df44a12f48
ce800f7f6e5764
server_private_key: f5002870ce2c5117d0ada53bf11fd7144f72510098b8d477f
ba67a07e5d1640b
server_public_key: 463499017daf13c3915d866656576a8920e15aaf860568d68d
4e1edbc5452802
server_nonce: 7954ebf0e81a893021ee24acc35e1a3f4b5e0366c15771133082ec2
1035ae0ef
server_keyshare: 42a889e6b6d90e31ce452e2ecf4d14ec0c5f5205981d828ae380
90fdcae8bc25
server_private_keyshare: 8d39667010ba488071c889447a547931809e3723b66e
33cf672395a8b48b980a
masking_key: 1bad1c8b6ad879e348e15bd698ee70b2c51d3e89d9c08b00889a1fa8
f3947a48dac9ad994e946f408a2c31250ee34f9d04a7d85661bab11c67048ecfb7a68
c65
KE1: 943a149cf304878367fa2dce5cb30eac23cfd1358e5cc0efdbd4361a9e7bd72d
c26fead2a8b3d5910e25fd29402530b5c7e852585f843f3b939993624b8a7c3b58106
2b0e8e90db4798adbb49581f016034e0855b6d6199aceb56a71c9bd4866
~~~

#### Output Values

~~~
KE2: d0b9756ee8cdf900c4120b84b2fcb9c1961b4272fa7d393a33ffa273587f547a
59d26775ae953b9552fdfbf2ab6f469f2f153f9a88aacb7ed434aed9fd7ac1abd4709
ea9ca3ff1bd89374fc6132b7c027593dc7f0ce02da216dde6e90c9a75da99aea9e1e6
c8de0cdd29d54df90584fa83be96c22436b80aed30ea658c79c40cee00730b9d866c2
db24145be6911b530631f5e279ee3fb0b801c3d0c0c3c7de54c745f219fde845a5b9c
415facd45b670dea221104a2a73dab32a0cd951d20a67954ebf0e81a893021ee24acc
35e1a3f4b5e0366c15771133082ec21035ae0ef42a889e6b6d90e31ce452e2ecf4d14
ec0c5f5205981d828ae38090fdcae8bc258ce81724ca613428f82a6f9376f03904f34
dd85794caaaafb55abedfddd35e785e5f7543a8b52964b290869bdd9b786d2c412e50
72784ee403eee9acfa8a5b9e
~~~

### OPAQUE-3DH Fake Test Vector 2

#### Configuration

~~~
OPRF: 0003
Hash: SHA256
KSF: Identity
KDF: HKDF-SHA256
MAC: HMAC-SHA256
Group: P256_XMD:SHA-256_SSWU_RO_
Context: 4f50415155452d504f43
Nh: 32
Npk: 33
Nsk: 32
Nm: 32
Nx: 32
Nok: 32
~~~

#### Input Values

~~~
client_identity: 616c696365
server_identity: 626f62
oprf_seed: 94384ca183c8e6f639ab29b5d2a81ef4305df9a67cb33db5ba8082e4f4
bfb830
credential_identifier: 31323334
masking_nonce: 375d7dcbd562a62190cc569ccc809cff9d5aa5e176d48e9646b558
eb41ffab7c
client_private_key: 9ecb5dc678e429e1a01ad6fe5d45301484d12c2a2cf2278fa
c0a0a2cf96eef57
client_public_key: 035951b821e6e1e449933fdba30c7e2e8b6e8f42f4c7a54c80
010a339e72cb2253
server_private_key: 7154525469c4fbae6c9907f4ff26a6386c0a4077f512138e2
203f247d56cbe91
server_public_key: 02acadb2750e036bfcbdb3c5aacad0f55c832631cc8f8e26a6
bc65f7e53525ae79
server_nonce: e0d04374ad9a276620c681abfca7bdb432e63509e5ec96ed2ec5542
f6fc7db23
server_keyshare: 02ffe33c6d938b4d10afeb4ad5ba108ad228317ecab3d6a78a3b
4e2494dc7ec8fb
server_private_keyshare: 034ce43e75362936d67055acf8301fbb910e2afd8769
c4334721fc4ff6bab1d7
masking_key: 1db20e37f4539b2327d37b80c00b98a2cfea9156e5e889b4efa3556e
9f0d24ce
KE1: 0258384d63ae4bbddde6d00d41b0e7174695ff6234563e16fc284aa589c7de93
f9b8bb2700cdd47e339d95404519f2fb3da58c93d84cbb4d51de6757a31919382b026
30e46a94b7f8f66071d24794c37f605055c098afc04d637caf9b1bc714bd15c
~~~

#### Output Values

~~~
KE2: 02d0866ee88ab8dc2eb5e39e859e55fa96dca50dc2d280e66dc747f21a14c015
f9375d7dcbd562a62190cc569ccc809cff9d5aa5e176d48e9646b558eb41ffab7c7d6
5027823b4a13e0a19c738eed6ccf3ee141697d93ffe7192a32d1cf803557cc1627fe1
1ccc933dffb662c2d8bbaa97c375287cb172d942c0f0252be71c74f367bd69bc17a7a
7d1e5e25dd1528e81a65f3b8a266c45f0dbf66486c1c65749ff06e0d04374ad9a2766
20c681abfca7bdb432e63509e5ec96ed2ec5542f6fc7db2302ffe33c6d938b4d10afe
b4ad5ba108ad228317ecab3d6a78a3b4e2494dc7ec8fb120aed0e35ab8f67a2a723fe
bf5e5f590d57c08245419972555a59b058240c46
~~~
"#;
