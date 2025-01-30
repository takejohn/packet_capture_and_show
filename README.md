# パケットキャプチャことはじめ
Rustで[`pnet`](https://crates.io/crates/pnet)クレートを使用してパケットキャプチャをやる。

## Usage
```
cargo run -- <ネットワークインターフェース名>
```

## Windows
Windowsの場合、以下の3つの依存関係が必要。  
参考: https://github.com/libpnet/libpnet?tab=readme-ov-file#windows

### MSVCツールチェイン
MSVCツールチェインを使用するRustのバージョンが必要。

### WinPcapまたはNpcap
WinPcapまたはNpcapがインストールされている必要がある。
以下はNpcapを使用する場合の説明。
1. [Npcapのダウンロードページ](https://npcap.com/#download)から最新版のNpcap installerをダウンロードする
   (2025年1月30日現在、バージョンは「Npcap 1.80 installer」となっている)。
1. ダウンロードしたNpcap installer (npcap-1.80.exe)を起動する。
1. 規約を読み、同意する場合は「I Agree」をクリックする。
1. 「Install Npcap in WinPcap API-compatible Mode」にチェックが入っていることを確認して、「Install」をクリックする。
1. インストールが終わったら、「Next >」をクリックし、「Finish」をクリックしてインストーラを終了する。
1. Npcap SDK (npcap-sdk-1.13.zip)を解凍する。

### `Packet.lib`
`Packet.lib`をRustのツールチェインから参照できるようにする必要がある。
以下はNpcap SDKを使用する場合の説明。
1. [Npcapのダウンロードページ](https://npcap.com/#download)から最新版のNpcap SDKをダウンロードする
   (2025年1月30日現在、バージョンは「Npcap SDK 1.13」となっている)。
1. ダウンロードしたZIPファイル(npcap-sdk-1.13.zip)を展開 (例: `C:\npcap-sdk-1.13`)。
1. システムの環境変数(設定＞システム＞バージョン情報＞システムの詳細設定＞環境変数)に以下の値を追加する。
   - `LIB`: 展開したフォルダ内の`Lib\x64`フォルダのパス (例: `C:\npcap-sdk-1.13\Lib\x64`)
     - ターゲットがx64でなくx86 (32 bitアーキテクチャ)なら`Lib`フォルダのパス
   - `INCLUDE`: 展開したフォルダ内の`Include`フォルダのパス (例: `C:\npcap-sdk-1.13\Include`)
