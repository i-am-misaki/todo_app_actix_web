/**
 * 削除確認用のモーダルを表示する。
 * ユーザーが「削除」を確定した場合のみ、実際の削除APIを呼び出す。
 *
 * @param {number} taskId - 削除対象となるタスクのID
 */
function DeleteConfirm(taskId){
    Swal.fire({
      title: "本当に削除していいと？",
      text: "元に戻せんよ！",
      icon: "warning",
      showCancelButton: true,
      confirmButtonColor: "#3085d6",
      cancelButtonColor: "#d33",
      confirmButtonText: "削除すっぞ",
      cancelButtonText: "やっぱやめる"
    }).then((result) => {
      if (result.isConfirmed) {
        DeleteTaskApi(taskId)
      }
    });
}


/**
 * 指定されたタスクIDをサーバーに送信し、タスクを削除する。
 * 削除完了後は一覧を再読み込みする。
 *
 * @param {number} taskId - 削除対象となるタスクのID
 */
function DeleteTaskApi(taskId){
    fetch('/todo/delete', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/x-www-form-urlencoded'
        },
        body: `id=${taskId}`
    }).then(() => {
        console.log('タスクは削除されました。');
        location.reload();
    }).catch(err => {
        console.error(err);
    });
}
