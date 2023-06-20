from gensim.models import KeyedVectors
from simplemma import lemmatize


class Model:
    def __init__(self, model_path) -> None:
        self.model_path = model_path
        try:
            self.model = KeyedVectors.load_word2vec_format(
                model_path, binary=True, unicode_errors="ignore"
            )
        except Exception as e:
            print(f"Exception: {e}")

    def _most_similar(self, word):
        l = self.model.most_similar(word)
        print(l)
        lemmatized_list = []
        for i in l:
            if i[0].endswith("é"):
                lemmatized_list.append(i[0])
            else:
                lemmatized_list.append(lemmatize(i[0], lang="fr"))
        if word in lemmatized_list:
            lemmatized_list.remove(word)
        return list(dict.fromkeys(lemmatized_list))
