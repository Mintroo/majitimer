# Maji Timer (本気タイマー)

Maji Timerは、頑張った分だけ休憩が非線形におおよそ多くなるタイマーアプリケーションです。やるまでのやる気が出ないもどかしさを軽減する目的で作成しました。

## 機能

- **本気モード**: 短時間の集中作業
- **耐久モード**: 長時間の作業セッション
- **休憩モード**: 作業時間に応じた休憩
- **カスタマイズ可能な設定**: タイマーの動作をユーザーのニーズに合わせて調整可能
- **音声通知**: モード切替時やリマインダーとして使用可能

## インストール

```
git clone https://github.com/Mintroo/majitimer.git
cd majitimer
cargo build --release
```

もしくは、配布されているバイナリを直接実行してください。(Windows-x64限定)

## 使用方法

1. スペースキーを押して、タイマーを開始します。

2. 以下のキーを使用して、アプリケーションを操作します：
- `Space`: ポーズ/再開
- `R`: リセット
- `M`: モード移行のインタラクト
- `I`: 設定のインポート
- `E`: 設定のエクスポート
- `Q`: 終了

以下のキーバインドはTUI上で表示されません。注意してください。

- `1`: `finish_sound`として再生する音声ファイルのパスの設定
- `2`: `restart_sound`として再生する音声ファイルのパスの設定
- `3`: `remind_sound`として再生する音声ファイルのパスの設定

## カスタマイズ

初回起動時に`.config/majitimer/config.json`ファイルが自動生成されます。

`config.json`を直接編集することで各種設定を行えます。

休憩時間の算出に使われるパラメーターの説明に関しては、[memo/memo.txt](https://github.com/Mintroo/majitimer/blob/main/memo/memo.txt)を参照してください。

- `finish_sound`: 本気モード終了時に再生されるサウンド
- `restart_sound`: 休憩モード終了時に再生されるサウンド
- `remind_sound`: 休憩モード終了後のリマインダーとして再生されるサウンド

また、設定ファイルのインポート、エクスポート機能にも対応しています。

## ライセンス

このプロジェクトは[MITライセンス](LICENSE)の下で公開されています。
