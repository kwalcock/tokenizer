# In [1]:
import random
import numpy as np
import torch
from tqdm import tqdm
# from tqdm.notebook import tqdm

# Linux
# base_dir = '/media/kwa/Data Disk/home/kwa/Projects/corpora/aclImdb/'
# Mac
# base_dir = '/Users/kwa/Projects/kwalcock/m1-dlnlp-project/corpora/aclImdb/'
# Windows
base_dir = 'E:/DocumentCollections/aclImdb/'

# set this variable to a number to be used as the random seed
# or to None if you don't want to set a random seed
seed = 1234

if seed is not None:
    random.seed(seed)
    np.random.seed(seed)
    torch.manual_seed(seed)

# In [4]:
from glob import glob

pos_files = glob(base_dir + 'train/pos/*.txt')
neg_files = glob(base_dir + 'train/neg/*.txt')

print('number of positive reviews:', len(pos_files))
print('number of negative reviews:', len(neg_files))

# In [5]:
from sklearn.feature_extraction.text import CountVectorizer

# initialize CountVectorizer indicating that we will give it a list of filenames that have to be read
cv = CountVectorizer(input='filename')

# learn vocabulary and return sparse document-term matrix
doc_term_matrix = cv.fit_transform(pos_files + neg_files)
doc_term_matrix

# In [6]:
X_train = doc_term_matrix.toarray()
X_train.shape

# In [7]:
# training labels
y_pos = np.ones(len(pos_files))
y_neg = np.zeros(len(neg_files))
y_train = np.concatenate([y_pos, y_neg])
y_train

# In [8]:
n_examples, n_features = X_train.shape

# In [9]:
import torch
from torch import nn
from torch import optim

lr = 1e-1
n_epochs = 10

model = nn.Linear(n_features, 1)
loss_func = nn.BCEWithLogitsLoss()
optimizer = optim.SGD(model.parameters(), lr=lr)

X_train = torch.tensor(X_train, dtype=torch.float32)
y_train = torch.tensor(y_train, dtype=torch.float32)

indices = np.arange(n_examples)
for epoch in range(n_epochs):
    # n_errors = 0
    # randomize training examples
    np.random.shuffle(indices)
    # for each training example
    for i in tqdm(indices, desc=f'epoch {epoch+1}'):
        x = X_train[i]
        y_true = y_train[i]
        # make predictions
        y_pred = model(x)
        # calculate loss
        loss = loss_func(y_pred[0], y_true)
        # calculate gradients through back-propagation
        loss.backward()
        # optimize model parameters
        optimizer.step()
        # ensure gradients are set to zero
        model.zero_grad()

# In [10]:
pos_files = glob(base_dir + 'test/pos/*.txt')
neg_files = glob(base_dir + 'test/neg/*.txt')
doc_term_matrix = cv.transform(pos_files + neg_files)
X_test = doc_term_matrix.toarray()
X_test = torch.tensor(X_test, dtype=torch.float32)
y_pos = np.ones(len(pos_files))
y_neg = np.zeros(len(neg_files))
y_test = np.concatenate([y_pos, y_neg])

# In [11]:
y_pred = model(X_test) > 0

# In [12]:
def binary_classification_report(y_true, y_pred):
    # count true positives, false positives, true negatives, and false negatives
    tp = fp = tn = fn = 0
    for gold, pred in zip(y_true, y_pred):
        if pred == True:
            if gold == True:
                tp += 1
            else:
                fp += 1
        else:
            if gold == False:
                tn += 1
            else:
                fn += 1
    # calculate precision and recall
    precision = tp / (tp + fp)
    recall = tp / (tp + fn)
    # calculate f1 score
    fscore = 2 * precision * recall / (precision + recall)
    # calculate accuracy
    accuracy = (tp + tn) / len(y_true)
    # number of positive labels in y_true
    support = sum(y_true)
    return {
        "precision": precision,
        "recall": recall,
        "f1-score": fscore,
        "support": support,
        "accuracy": accuracy,
    }

# In [13]:
print(binary_classification_report(y_test, y_pred))
