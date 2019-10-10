
// 表示するデータ:初期表示用
let fieldData = 
    "0";
let xWidth = 640;
let yHeight = 480;

// キャンバスを取得する
let canvas = document.getElementById("canvas_field");

// キャンバスのサイズを変更
canvas.width = xWidth + 2;
canvas.height = yHeight + 2;

// ボックスなどを描画するメソッドを取得
let context = canvas.getContext("2d");

// キャンバス全体を白で塗りつぶす
context.fillStyle = "#FFFFFF";
context.fillRect(0, 0, canvas.width, canvas.height);

//枠をつける
context.strokeStyle = "#888888";
context.strokeRect(0, 0, canvas.width, canvas.height);

// 初期処理はここまで

//実行
function drawField(){
	document.getElementById('targetText').innerHTML = "実行中です…";

	// 表示するデータ:入力から取得
	let input = document.forms.id_mainform.id_textArea_field.value;
	let inputArr = input.split(/\r\n|\r|\n/);
	
	
	// ★フィールドの初期表示
	let fieldStr = inputArr[0].split(";");
	if(fieldStr.length < 3)
	{
		document.getElementById('targetText').innerHTML = "フィールド定義が不正です";
		return;
	}
	
	let fieldData = 
	    fieldStr[2];
	let xWidth = parseInt(fieldStr[0], 10);
	let yHeight = parseInt(fieldStr[1], 10);
	
	if(xWidth > 640 || yHeight > 480 || xWidth < 32 || yHeight < 32)
	{
		document.getElementById('targetText').innerHTML = "フィールドサイズが不正です";
		return;
	}

	// キャンバスを取得する
	let canvas = document.getElementById("canvas_field");

	// キャンバスのサイズを変更
	canvas.width = xWidth + 2;
	canvas.height = yHeight + 2;

	// ボックスなどを描画するメソッドを取得
	let context = canvas.getContext("2d");

	// キャンバス全体を白で塗りつぶす
	context.fillStyle = "#FFFFFF";
	context.fillRect(0, 0, canvas.width, canvas.height);

	//枠をつける
	context.strokeStyle = "#888888";
	context.strokeRect(0, 0, canvas.width, canvas.height);

	// 幅と高さを指定してImageDataを作成する
	let imageData = context.createImageData(xWidth, yHeight);

	// ImageDataに絵のデータを設定する
	// 縦横サイズは合っているので改行は考えず順番に表示するだけでよい
	let fieldDataArr = fieldData.split("");
	if(fieldDataArr.length != xWidth * yHeight){
		document.getElementById('targetText').innerHTML = "フィールド定義が不正です";
		return;
	}
	
	for (let i = 0; i < fieldDataArr.length; i++) {
	  switch(fieldDataArr[i])
	  {
	    case "1": // 黒
	      imageData.data[i * 4 + 0] = 0; // 赤
	      imageData.data[i * 4 + 1] = 0; // 緑
	      imageData.data[i * 4 + 2] = 0; // 青
	      imageData.data[i * 4 + 3] = 255; // アルファチャンネル
	      break;
	    case "0": // 白
	      imageData.data[i * 4 + 0] = 255; // 赤
	      imageData.data[i * 4 + 1] = 255; // 緑
	      imageData.data[i * 4 + 2] = 255; // 青
	      imageData.data[i * 4 + 3] = 255; // アルファチャンネル
	      break;
	    default:
	      document.getElementById('targetText').innerHTML = "フィールド定義が不正です";
		  return;
	  }
	}

	// ImageDataを作画する
	context.putImageData(imageData, 1, 1);


    // ★スタンプ一覧を描画する
    // スタンプのデータを取ってくる
    
    let stamps = [];
    let totalWidth = 0;
    let maxHeight = 0;
    
    for(let i = 1; i < inputArr.length; i++){
    	let newStamp = inputArr[i].split(";")
    	if(newStamp.length < 3 && inputArr[i].length > 0){
    		document.getElementById('targetText').innerHTML = i + "番目のスタンプの定義が不正です";
			return;
    	}
    	if(inputArr[i].length == 0)
    	{
    		break;
    	}
    	newStamp[0] = parseInt(newStamp[0], 10);
    	newStamp[1] = parseInt(newStamp[1], 10);
    	if(newStamp[0] < 1 || newStamp[0] > 64 || newStamp[1] < 1 || newStamp[1] > 64){
	    	document.getElementById('targetText').innerHTML = i + "番目のスタンプの大きさが不正です";
			return;
		}
    	totalWidth = totalWidth + newStamp[0] + 3; // 枠のために+2px,隙間のために+1px
    	maxHeight = Math.max(newStamp[1], maxHeight)
    	
    	for(let j = 0; j < stamps.length; j++){
    		// スタンプ重複チェック
    		if(stamps[j][2] == newStamp[2]){
    			document.getElementById('targetText').innerHTML = "スタンプの定義が重複しています";
				return;
    		}
    	}
    	
    	// リストに入れる
    	stamps.push(newStamp); //3カラムの配列の配列になる
    }
    
    if(stamps.length < 2 || stamps.length > 64){
    	document.getElementById('targetText').innerHTML = "スタンプは2個以上64個以下としてください";
		return;
    }
    
    // キャンパス分けなくてもよかったが……
	let canvas_s = document.getElementById("canvas_stamps");

	// キャンバスのサイズを変更
	if(totalWidth > 640){
		// 改行が必要なケース
		canvas_s.width = 640;
		// 行数は、キリが悪いとはみ出してしまうかもしれないので一応1行余裕を持たせる
		let gyosu = Math.floor(totalWidth / 640) + 2;
		canvas_s.height = gyosu * (maxHeight + 3); // 枠のために+2px,隙間のために+1px
	}else{
		// 改行なしで横に並べられるケース
	    canvas_s.width = totalWidth;
	    canvas_s.height = maxHeight + 2; // 枠のために+2px
	}

	// ボックスなどを描画するメソッドを取得
	let context_s = canvas_s.getContext("2d");

	// キャンバス全体を白で塗りつぶす
	context_s.fillStyle = "#FFFFFF";
	context_s.fillRect(0, 0, canvas_s.width, canvas_s.height);
	
	//枠は灰色
	context_s.strokeStyle = "#888888";
    
    // ImageDataを一つずつ作って、いい感じの位置にputしていく
    // 描画のスタート位置
    let tempX = 1;
    let tempY = 1;
    
    for(let i = 0; i < stamps.length; i++){
    	let stamp = stamps[i];
    	// 幅と高さを指定してImageDataを作成する
		let imageData = context_s.createImageData(stamp[0], stamp[1]);
		
    	let stampDataArr = stamp[2].split("");
    	if(stampDataArr.length != stamp[0] * stamp[1]){
    		document.getElementById('targetText').innerHTML = "スタンプ定義が不正です";
			return;
    	}
    	
		for (let i = 0; i < stampDataArr.length; i++) {
		  switch(stampDataArr[i])
		  {
		    case "1": // 黒
		      imageData.data[i * 4 + 0] = 0; // 赤
		      imageData.data[i * 4 + 1] = 0; // 緑
		      imageData.data[i * 4 + 2] = 0; // 青
		      imageData.data[i * 4 + 3] = 255; // アルファチャンネル
		      break;
		    case "0": // 白
		      imageData.data[i * 4 + 0] = 255; // 赤
		      imageData.data[i * 4 + 1] = 255; // 緑
		      imageData.data[i * 4 + 2] = 255; // 青
		      imageData.data[i * 4 + 3] = 255; // アルファチャンネル
		      break;
		    default: // それ以外は白
	        document.getElementById('targetText').innerHTML = "スタンプ定義が不正です";
		    return;
		  }
		}

		// この行からはみ出す場合は次の行
		if (tempX + stamp[0] > 640){
			tempX = 1;
			tempY = tempY + maxHeight + 3;
		}

		// ImageDataを作画する
		context_s.putImageData(imageData, tempX, tempY);
		
		// 枠を描く
		context_s.strokeRect(tempX - 1, tempY - 1, stamp[0] + 2, stamp[1] + 2);
		
		tempX = tempX + stamp[0] + 3; // 次の描画スタート位置
	}
    
    // ★スタンプの描画
    
    //出力を読み込む
    let output = document.forms.id_mainform.id_textArea_answer.value;
    let outputArr = output.split(/\r\n|\r|\n/);
    let num = parseInt(outputArr[0], 10);
    
    let stampedDataArr = Array(fieldDataArr.length); // fieldDataArrと同じサイズ
    for(let i = 0; i < stampedDataArr.length; i++)
    {
    	stampedDataArr[i] = "0";
    }
    
    if(num == 0)
    {
    	calcPoint(fieldDataArr, stampedDataArr, 0);
		return;
    }
    
    let putStamps = outputArr.slice(1, outputArr.length + 1);
    
    // 描画インターバル
    let interval = Math.floor(3000 / num);
    if (interval > 1000){
    	interval = 1000;
    }
    
    //スタンプを押していく
    let i = -1; //諸事情により
    let id = null;
    
    function drawStamp(putStamps, stampedDataArr, fieldDataArr, imageData, context){
    	i++;
		if(i >= num){
		    // 最後に採点する
		    calcPoint(fieldDataArr, stampedDataArr, num);
			clearInterval(id);
			return;
		}
		
		let stampArr = putStamps[i].split(";");
		if(stampArr.length < 2){
			document.getElementById('targetText').innerHTML = "回答の" + (i + 2) + "行目に不正な値があります";
			clearInterval(id);
			return;
		}
		let stampPos = stampArr[1].split(",");
		if(stampPos.length < 2){
			document.getElementById('targetText').innerHTML = "回答の" + (i + 2) + "行目に不正な値があります";
			clearInterval(id);
			return;
		}
		
		if(isNaN(parseInt(stampArr[0])) || isNaN(parseInt(stampPos[0])) || isNaN(parseInt(stampPos[1]))){
			document.getElementById('targetText').innerHTML = "回答の" + (i + 2) + "行目に不正な値があります";
			clearInterval(id);
			return;
		}
		
		stampPos[0] = parseInt(stampPos[0], 10);
		stampPos[1] = parseInt(stampPos[1], 10);
		stampArr[0] = parseInt(stampArr[0], 10);
		
		if(stampPos[0] + stamps[stampArr[0]][0] - 1 < 0 || stampPos[0] >= xWidth || stampPos[1] + stamps[stampArr[0]][1] - 1 < 0 || stampPos[1] >= yHeight){
			document.getElementById('targetText').innerHTML = "回答の" + (i + 2) + "行目のスタンプ位置が不正です";
			clearInterval(id);
			return;
		}
		
		let pos = getBlackPositions(stamps[stampArr[0]], xWidth, yHeight, stampPos[0], stampPos[1]);
		
		for (let j = 0; j < pos.length; j++){
			let tempS = stampedDataArr[pos[j]];
			switch(tempS){
				case "0":
					stampedDataArr[pos[j]] = "1";
					break;
				case "1":
					stampedDataArr[pos[j]] = "0";
					break;
				default:
					document.getElementById('targetText').innerHTML = "フィールド定義に不正な値があります";
					clearInterval(id);
					return;
			}
		}
		
		// 一回ごとに描画する
		for (let i = 0; i < stampedDataArr.length; i++) {
			//stampedDataArr[i]とfieldDataArr[i]の組み合わせで出力
			//アルファチャンネルは変更ないはずなので省略
			if(stampedDataArr[i] == "1" && fieldDataArr[i] == "1")
			{
				// 両方1なら黒
				imageData.data[i * 4 + 0] = 0; // 赤
				imageData.data[i * 4 + 1] = 0; // 緑
				imageData.data[i * 4 + 2] = 0; // 青
			}
			else if(stampedDataArr[i] == "0" && fieldDataArr[i] == "0")
			{
				// 両方0なら白
				imageData.data[i * 4 + 0] = 255; // 赤
				imageData.data[i * 4 + 1] = 255; // 緑
				imageData.data[i * 4 + 2] = 255; // 青
			}
			else if(stampedDataArr[i] == "1" && fieldDataArr[i] == "0")
			{
				// お手本にないところがスタンプで塗られている場合、赤
				imageData.data[i * 4 + 0] = 255; // 赤
				imageData.data[i * 4 + 1] = 153; // 緑
				imageData.data[i * 4 + 2] = 0; // 青
			}
			else if(stampedDataArr[i] == "0" && fieldDataArr[i] == "1")
			{
				// お手本にあるところがスタンプで塗られていない場合、青
				imageData.data[i * 4 + 0] = 0; // 赤
				imageData.data[i * 4 + 1] = 65; // 緑
				imageData.data[i * 4 + 2] = 255; // 青
			}
			else
			{
				document.getElementById('targetText').innerHTML = "描画中にエラーが発生しました";
				clearInterval(id);
				return;
			}
		}

		// ImageDataを作画する
		context.putImageData(imageData, 1, 1);
		
	}
    
	id = setInterval(drawStamp, interval, putStamps, stampedDataArr, fieldDataArr, imageData, context);
}

function calcPoint(baseArr, stampedArr, tekazu)
{
	let count = 0;
	
	if(baseArr.length != stampedArr.length){
	    document.getElementById('targetText').innerHTML = "結果計算時にエラーが発生しました";
		return;
	}
	
	for(let i = 0; i < baseArr.length; i++)
	{
		if(baseArr[i] != stampedArr[i]){
			count = count + 1;
		}
	}

	document.getElementById('targetText').innerHTML = "相違は" + count + "セル, スタンプ回数は" + tekazu + "回です";
	return;
}

function getBlackPositions(stamp, Fwidth, Fheight, sX, sY)
{
	// stampは[幅,高さ,模様]の配列
	// Fwidth, Fheightはフィールドの幅と高さ
	// sX, sYはスタンプの左上の座標
	// 反転させる位置はフィールド左上から数えて何セル目？ というリストを返す
	
	ans = []
	
	for(let i = 0; i < stamp[2].length; i++)
	{
		if(stamp[2][i] != "1")
		{
			continue;
		}
		
		// スタンプ左上からの相対位置
		relY = Math.floor(i / stamp[0]);
		relX = i - (relY * stamp[0]);
		
		// 座標
		absX = relX + sX;
		absY = relY + sY;
		
		// フィールドからはみ出す場合は不要
		if(absX < 0 || absY < 0 || absX >= Fwidth || absY >= Fheight)
		{
			continue;
		}
		
		// セル番号
		let temp = absY * Fwidth + absX;
		
		ans.push(temp);
	}
	
	return ans;
}

