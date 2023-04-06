# Dove: Advance Donation using the blockchain
<img src="./images/logo.png" width="50%">

## Background
There are disasters, wars, and other matters that people wish to avoid beyond borders. Some of them are human-caused disasters that can be (still) delayed or avoided through human intervention, such as acts of invasion or rising temperatures caused by greenhouse effect gases. When these disastrous events (regrettably) occur, people sympathize, and in the spirit of mutual aid, one form of help is to donate money.

However, it is best to avoid the situation in the first place. What if there was an economic incentive to prevent incidents and continue peace or protect the global environment using donations? Furthermore, what if the amount of this donation had a deterrent effect on the other party? This project aims to provide incentives for continued contributions and to deter destructive actions by asking donations before the undesired event and withdrawing the fund when the event has been confirmed that it indeed happened.

### 1. The Basic Idea
First, the project will provide a place where people can donate depending on how much they wish to avoid a specific “incident.” Anyone can participate in this donation. Now, let us call the collected funds “advance donation.” If the “incident” actually occurs, an organization (aid group) that works to minimize the damage will receive this advance donation. Depending on the amount of donation, and knowing that the advance donation will go to an aid group may discourage the counter party. This aid group could be the invaded country or an NGO that promotes greenhouse gas mitigation. While this “incident” does not occur, the advance donation can be used for investment and its return could be used to provide further support to the aid group or reimburse donors for their contributions. While doners are already willing to donate, returns will incentivize those who wish to avoid the “incident” and continue to do so.

#### 1.1. Criteria of the “Incident”
There are two criteria for the ’incident’ in order to work. First, the “incident” should be avoidable by human effort. For example, earthquakes and tsunamis, for which there is no fundamental method to avoid, are not suitable. Where as acts of aggression between nations and rising temperatures due to greenhouse effects may be a fit. Second, the “incident” should be easy to confirm based on factual information. For example, “a landing of a missile” or “the difference in average temperatures in various regions.” However, examples show that facts get artificially manipulated via propaganda, so whether or not the “incident” has occurred will be determined by voting from the donor community.

#### 1.2 Transparancy and DAO
Regular donation projects require trust, and to build this trust, transparency of whether the donations got transferred correctly and the usage (not misappropriated) is critical. This project similarly requires trust during fund aggregation yet the trusted entity will be replaced with a smart contract. Transparency when voting for advance donation transfers, can also be met with this smart contract. In other words, it would be a DAO (Decentralized Autonomous Organization) for preventing man-caused catastrophes.

In addition to the approval of donation transfer, other topics also need consensuses, such as the ratio of votes required for approval (majority, 2/3, etc.), the selection of the recipient (which aid group?), and the vote collection processes. The DAO will also be the place for such discussions.

### 2 Mile stones
#### 2.1 MVP and the frontend
First, as an MVP, we will develop a place to aggregate advance donations and a voting mechanism to approve withdrawals. The key for this phase is to deploy the smart contract on devnet and develop the front end. The project will use the Solana blockchain 1, which is cheaper and faster than Ethereum. The system will use SOL (the primary currency used in the Solana blockchain) at this stage.

#### 2.2 Further DAO development
After the essential processes are in place, we will augment its function as a DAO. It should be able to handle quorum, percentage of affirmative votes required for withdrawal, amount of donations accepted, and operational policy. We will then have the smart contract audited to be prepared to be operated on the mainnet. We will also consider issuing the project’s own currency to control the number of donations and to decouple with the SOL price.(The base blockchain will continue to be Solana.)

#### 2.3 Lowening the barrier
The act of redeeming at a virtual currency exchange and installing a wallet on a device (or browser) for transactions on the blockchain is still considered a high barrier to participation. This is not limited to Solana.

Therefore, after the DAO is established, infrastructure to lower the participation barrier is important. Specifically, this will make it it easier to participate using legal tender such as dollars, euros and yen. The organization responsible for this function of connecting the blockchain to the common people will need to be legally registered.

***

## Development
### Make your own branch and setup the enviornment
 1. Fork the repo to your account and clone the forked repo to your local laptop.
 1. Install the dependency of `@project-serum/anchor`
    > yarn add @project-serum/anchor
 1. Generate your wallet to deploy the program to Solana block chain.
    > solana-keygen new -o ./id.json
 1. Check the address of your wallet.
    > solana address
    > solana address -k ./id.json
 1. Airdrop at least 6 SOL to deploy the program to the localnet.
    > solana airdrop 2
    > solana airdrop 2
    > solana airdrop 2 `YOUR WALLET ADDRESS`
 1. Then build the program with Anchor.
    > anchor build
 1. Check the Program Account address.
    > solana address -k ./target/deploy/dove-keypair.json
 1. Copy & paste your Program Account addresses. E.g., if the address was `HCe8d6dZzxnLGuqtiKNERShgnVSVf6txrDmyCQEQdmTN`, you should update the addresses in the below three parts of the files.
    - ./Anchor.toml
        
      ```
      [programs.localnet]
      dove = "HCe8d6dZzxnLGuqtiKNERShgnVSVf6txrDmyCQEQdmTN"
      
      [programs.devnet]
      dove = "HCe8d6dZzxnLGuqtiKNERShgnVSVf6txrDmyCQEQdmTN"
      ```
      
    - ./programs/dove/src/lib.rs
      ```
      use instructions::*;
      
      declare_id!("HCe8d6dZzxnLGuqtiKNERShgnVSVf6txrDmyCQEQdmTN");
      
      #[program]
      pub mod dove {
      ```
      
 1. Rebuild the program again with the updated address.
    > anchor build
 1. If the error occurred by the inconsistency with the cached key pair, recover/update the cached key pair with the below command and the shown 12-word seed phrase.
    > solana-keygen recover --force
    - The error message could be:
    
      ```
      Deploying workspace: http://localhost:8899
      Upgrade authority: ./id.json
      Deploying program "dove"...
      Program path: /home/ohsugi/dove-dev/target/deploy/dove.so...
      =====================================================================
      Recover the intermediate account's ephemeral keypair file with
      `solana-keygen recover` and the following 12-word seed phrase:
      =====================================================================
      coyote twin dish round acid talk marble arch stuff review turn unique
      =====================================================================
      To resume a deploy, pass the recovered keypair as the
      [BUFFER_SIGNER] to `solana program deploy` or `solana program write-buffer'.
      Or to recover the account's lamports, pass it as the
      [BUFFER_ACCOUNT_ADDRESS] argument to `solana program close`.
      =====================================================================
      Error: Deploying program failed: Error processing Instruction 1: custom program error: 0x1
      There was a problem deploying: Output { status: ExitStatus(unix_wait_status(256)), stdout: "", stderr: "" }.
      ```
      
    - In this case `12-word seed phrase` would be `coyote twin dish round acid talk marble arch stuff review turn unique`.

### Test on the Localnet
 - Run the Solana local validator node for testing in a console window.
    > solana-test-validator
 - Run Anchor Test by skipping to boot the local validator node in the other console.
    > anchor test --skip-local-validator

### Environment
#### Solana
 - Run the solana-installer in a console to catchup/apply the latest updates
    > solana-install update
 - Verify the installation.
    > solana --version

#### Anchor
 - Install the Anchor version manager that is a tool for using multiple versions of the anchor-cli. It will require the same dependencies as building from source. It is recommended you uninstall the NPM package if you have it installed. Install avm using Cargo. Note this will replace your anchor binary if you had one installed.
    > cargo install --git https://github.com/project-serum/anchor avm --locked --force
 - On Linux systems you may need to install additional dependencies if cargo install fails. E.g. on Ubuntu:
    > sudo apt-get update && sudo apt-get upgrade && sudo apt-get install -y pkg-config build-essential libudev-dev
 - Install the latest version of the CLI using avm, and then set it to be the version to use.
    > avm install latest
    > avm use latest
 - Verify the installation.
    > anchor --version

## Consideration
- The objective of the project-based funding system instead of the specific target country
  - 受取相手のWalletのPubkeyがわからないため、自己申告制にしてWalletのPubkeyを登録してもらう
  - アイデア、ユースケース、平和に興味の中心があるため、政治信条的には中立的立場で運用したい
  - 特定の国対象のシステムを作ってもカウンターとなる敵対国の同様のシステムがすぐに作られるであろうと思われるため
  - 出資を募る人はシステムをプロモーションするモチベーションがあるため、宣伝してくれるはず
- 利用を「平和維持」に限定する意図
  - 抽象化すれば、何かが実現された（例えば「特定の国が攻撃を受ける」ということが実現された）と出資者がみなせばファンドが実施される、後払いのクラウドファンディングであるとみなせる
  - アイデア自体は他のユースケースでも利用できるため、そのようにシステムを作ることも可能
  - 利用方法を限定した方が説明がシンプルで明確になり、利用者のとっつきがよいと思われる
  - 単なるクラファンであればBlockchainでやる必要性は薄まる
- プールされているSolanaの使い道
  - 例えばプールされている金額の半分を自動的にステーキングして利益を稼ぐ
  - 稼いだ利益から一部をマージンとして受け取り、開発費・運営費に回す
  - 残りは出資者に投資比率で案分して配布
- 下記で何度か触れている「自身がその本人だと証明するためのリンク」について
  - 出資を募る人間、出資する人間、いずれもWalletのPubkeyは示せても、当人のidentityを他社が容易に確認できない
  - 例えば自身がある政府の関係者、あるいは援助物資を届けるNGO/NPOだと名乗っているとしても、それを他社が容易に確認できない
  - ひとつの方法として、ソーシャルメディアで当該のプロジェクトや出資についてポストしてもらい、それへのリンクを貼ってもらうというのは参考になり得る
  - 当該のソーシャルメディアアカウントが当人、当団体であると信用できそうであれば、ある種の裏付けになると思われる

## Architecture
- Frontend
  - Node.js
  - React
- Backend
  - Rust
  - Anchor
- Blockchain
  - Solana
  - Phantom Wallet
- Deployment
  - Replit
  - GitHub

## Account Specification
- **DoveProject**
  - **admin_wallet**: Pubkey: Admin's Wallet
  - **evidence_link**: String: Hyper link to show the other users to make sure the admin's identity
  - **project_name**: String: Project Name
  - **target_country_code**: String: Target Country code (defined in the iso_country::Country)
  - **opponent_country_code**: String: Opponent Country code (defined in the iso_country::Country)
  - **description**: String: Project description
  - **created_date**: i64: Project created date (unix-time stap)
  - **update_date**: i64: Project last update date (unix-time stap)
  - **is_effective**: bool: Project Effective flag
  - **is_deleted**: bool: Project Delete flag 
  - **video_link**: String: Video link to describe the project as string (intended Youtube)
  - **amount_pooled**: u64: The current pooled amount (as Lamports)
  - **amount_transferred**: u64: The amount transferred so far (as Lamports)
  - **decision**: u64: The current decision for this project
  - **bump**: u8

- **DoveFund**
  - **project_pubkey**: Pubkey: The target project pubkey
  - **user_pubkey**: Pubkey: The founder's Wallet pubkey
  - **amount_pooled**: u64: The current pooled amount
  - **amount_transferred**: u64: The transferred amount so far
  - **decision**: f32: The decision percentage
  - **shows_user**: bool: If the user will be shown on the project webpage
  - **shows_pooled_amount**: bool: If the user's pooled amount on the project webpage
  - **shows_transferred_amount**: bool: If the user's transferred amount on the project webpage
  - **created_date**: i64: Fund craetion date (as Unix Time)
  - **update_date**: i64: Fund update date (as Unix Time)

- **DoveUser**
  - **user_wallet**: Pubkey: Wallet pubkey
  - **user_name**: String: User name
  - **social_media_link**: String: Social media links of the user
  - **evidence_link**: String: HTML link to prove own identity
  - **is_shown: bool**: The profile will be shown on each project webpage
  - **amount_pooled**: u64: The current pooled amount
  - **amount_transferred**: u64: The transferred amount so far
  - **created_date**: i64: User craetion date (as Unix Time)
  - **update_date**: i64: User update date (as Unix Time)

## Screen Specification
- ランディングページ: タイトル、ページの説明、この企画そのものへのSNSへのリンクなどと併せて登録されているプロジェクトの一覧を表示するページ
  - プロジェクトページへの遷移ボタン、表示されている各プロジェクトのファンドページへのボタンが表示される
  - プロジェクトが増えてきた場合は、プロジェクト一覧の検索・フィルタリング機能を追加する
- プロジェクトページ: プロジェクトを登録するページ。次のパラメータを設定できる
  - 対象国の名前
  - プロジェクト登録日
  - プロジェクト更新日
  - プロジェクト有効フラグ
    - 無効のプロジェクトは画面上に表示されるものの「無効」である旨が表示され、投資や情報の変更ができなくなる
  - プロジェクト削除フラグ
    - 削除されたプロジェクトは画面上に表示されないし情報が変更できなくなる
  - プロジェクトの説明
  - 資金受取後の利用用途
  - 動画もしくは画像へのリンク
    - 動画はYoutubeを想定
  - 自身のソーシャルメディアアカウント・Webページなどへのリンク（10個まで）
  - 資金受け取り組織・個人（自身）の名前
  - 資金受け取りWalletのPubkey
  - 自身がその組織だと証明するためのリンク
    - 自身のSNSアカウントでこのプロジェクトを引用して皆に出資を呼び掛けた上で、その投稿へのダイレクトリンクの登録を推奨、プロジェクト登録後に設定可能
  - 現在の合計プール金額（自動算出）
  - これまでの合計プール金額最大値
  - これまでの合計送金済金額
  - 被攻撃判断閾値（自動算出）
    - 0～100%の数値でこの数値以上の割合の出資者が「攻撃を受けている」と判断すればプールされている資金が資金受取Walletへ送金される
    - 各出資者が妥当であると思われる閾値を入力し、出資金額による加重平均を算出して決定、パーセンテージで下2桁四捨五入（e.g. 56.32%）
  - 攻撃を受けていると判断している出資者の割合（自動算出）
    - 0～100%の数値で現在の出資者の意見に出資金額で加重平均
    - 攻撃を受けていると判断されている間は出資金額がそのまま資金受取Walletにダイレクトに送金され続ける
- ファンドページ: プロジェクトにファンドしたり、取りやめたり、出資金額、意見を変更するページ。次のパラメータを設定できる
  - 対象プロジェクトの各種情報で必要そうなものがページに表示される
  - 出資金額
  - 被攻撃判断閾値
    - プロジェクトの被攻撃判断閾値を算出するための数値、defaultは50%
  - 現在攻撃を受けていると判断するか
    - 0 or 1 でなく、パーセンテージで設定可能、誰かに判断を委任することは現状ではできない（単に実装が複雑になると思われるため）
  - 自身のアカウント情報をプロジェクトページに表示するか否か、defaultはNo
  - 自身の投資金額をプロジェクトページに表示するか否か、defaultはNo
  - ウォッチリストへの追加、defaultはNo
    - メールアドレスが登録されている場合にプッシュで情報を受け取りたいかどうか
- ユーザページ: 接続しているWalletに関する情報（便宜的にユーザ情報と呼ぶ）を閲覧・変更するページ。次のパラメータを設定できる
  - ユーザ名（誰かと重複していても良い）
  - メールアドレス（登録されていればプッシュで情報を配信できるようになる、画面には表示しない）
  - 自身のSNSアカウント・Webページなどへのリンク（10個まで）
  - 自身がその本人だと証明するためのリンク
    - 自身のSNSアカウントでこのアカウントを引用して皆に出資を呼び掛けた上で、その投稿へのダイレクトリンクの登録を推奨
  - これまで出資したプロジェクトのPubkey一覧
  - 自身の情報を公開したいか否か、defaultはNo
  - 現在の合計プール金額（自動算出）
  - これまでの合計プール金額最大値
  - これまでの合計送金済金額
  
  ## Credit
- Arjun: https://github.com/LearnWithArjun
