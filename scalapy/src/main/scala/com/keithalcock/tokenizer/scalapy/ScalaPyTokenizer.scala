package org.clulab.dlnlp_book.apps

import me.shadaj.scalapy.py
import me.shadaj.scalapy.py.SeqConverters

import scala.collection.immutable.HashMap

class Python {
  val pyLen = py.Dynamic.global.len
  val pyList = py.Dynamic.global.list
  val pyDict = py.Dynamic.global.dict
  val pyFloat = py.Dynamic.global.float

  type PyAny = py.Any
}

object LogisticRegressionApp extends Python with App {
  // Linux
  // val baseDir = "/media/kwa/Data Disk/home/kwa/Projects/corpora/aclImdb/"
  // Mac
  // val baseDir = "/Users/kwa/Projects/kwalcock/m1-dlnlp-project/corpora/aclImdb/"
  // Windows
  val baseDir = "E:/DocumentCollections/aclImdb/"

  // In [1]:
  val random = py.module("random")
  val np = py.module("numpy")
  val torch = py.module("torch")
  val tqdm = py.module("tqdm") // .notebook")

  // set this variable to a number to be used as the random seed
  // or to None if you don't want to set a random seed
  val seedOpt = Some(1234) // None
  seedOpt.foreach { seed =>
    random.seed(seed)
    np.random.seed(seed)
    torch.manual_seed(seed)
  }

  // In [4]:
  val glob = py.module("glob")
  var posFiles = glob.glob(baseDir + "train/pos/*.txt")
  var negFiles = glob.glob(baseDir + "train/neg/*.txt")

  println(s"number of positive reviews: ${pyLen(posFiles)}")
  println(s"number of negative reviews: ${pyLen(negFiles)}")

  // In [5]:
  val sklearn = py.module("sklearn.feature_extraction.text")
  val cv = sklearn.CountVectorizer(input = "filename")
  var docTermMatrix = cv.fit_transform(posFiles + negFiles)
  // println(docTermMatrix)

  // In [6]:
  var xTrain = docTermMatrix.toarray()
  println(xTrain.shape)

  // In [7]:
  // training labels
  var yPos = np.ones(pyLen(posFiles))
  var yNeg = np.zeros(pyLen(negFiles))
  var yTrain = np.concatenate(pyList(Seq(yPos, yNeg).toPythonProxy))
  println(yTrain)

  // In [8]:
  val Seq(nExamples, nFeatures) = xTrain.shape.as[Seq[Int]]

  // In [9]:
  val lr = 1e-1f
  val nEpochs = 10

  val model = torch.nn.Linear(nFeatures, 1)
  val lossFunc = torch.nn.BCEWithLogitsLoss()
  val optimizer = torch.optim.SGD(model.parameters(), lr = lr)

  xTrain = torch.tensor(xTrain, dtype = torch.float32)
  yTrain = torch.tensor(yTrain, dtype = torch.float32)

  val indices = np.arange(nExamples)
  Range(0, nEpochs).foreach { epoch =>
    // randomize training examplesa
    np.random.shuffle(indices)
    // for each training exmaple
    val progressBar = tqdm.tqdm(indices, desc = s"epoch ${epoch + 1}")
    Range(0, nExamples).foreach { index =>
      val i = indices.bracketAccess(index)
      val x = xTrain.bracketAccess(i)
      val yTrue = yTrain.bracketAccess(i)
      // make predictions
      val yPred = model(x)
      // calculate loss
      val loss = lossFunc(yPred.bracketAccess(0), yTrue)
      // calculate gradients through back-propagation
      loss.backward()
      // optimize model parameters
      optimizer.step()
      // ensure gradients are set to zero
      model.zero_grad()
      progressBar.update(n = 1)
    }
    progressBar.close()
  }
  println()

  // In [10]:
  posFiles = glob.glob(baseDir + "test/pos/*.txt")
  negFiles = glob.glob(baseDir + "test/neg/*.txt")
  docTermMatrix = cv.transform(posFiles + negFiles)
  var xTest = docTermMatrix.toarray()
  xTest = torch.tensor(xTest, dtype = torch.float32)
  yPos = np.ones(pyLen(posFiles))
  yNeg = np.zeros(pyLen(negFiles))
  val yTest = np.concatenate(pyList(Seq(yPos, yNeg).toPythonProxy)).as[Seq[Float]]

  // In [11]:
  val yPred = {
    val result = model(xTest)
    val seq = result.detach().cpu().numpy().as[Seq[Float]]
    val ans = seq.map { value => if (value > 0f) 1f else 0f }

    ans
  }

  // In [12]:
  def binaryClassificationReport(yTrue: Seq[Float], yPred: Seq[Float]): Map[String, Float] = {
    // count true positives, false positives, true negatives, and false negatives
    var (tp, fp, tn, fn) = (0f, 0f ,0f, 0f)

    for ((gold, pred) <- yTrue.zip(yPred)) {
      if (pred == 1f)
        if (gold == 1f) tp += 1
        else fp += 1
      else
        if (gold == 0f) tn += 1
        else fn += 1
    }
    // calculate precision and recall
    val precision = tp / (tp + fp)
    val recall = tp / (tp + fn)
    // calculate f1 score
    val fscore = 2 * precision * recall / (precision + recall)
    // calculate accuracy
    val accuracy = (tp + tn) / yTrue.length
    // number of positive labels in yTrue
    val support = yTrue.sum

    HashMap(
      "precision" -> precision,
      "recall" -> recall,
      "f1-score" -> fscore,
      "support" -> support,
      "accuracy" -> accuracy
    )
  }

  println(binaryClassificationReport(yTest, yPred))
}
