from fastapi import FastAPI
from model import Model

app = FastAPI()

model_path = "frWac_non_lem_no_postag_no_phrase_200_cbow_cut100.bin"
model = Model(model_path=model_path)


@app.get("/")
def read_root():
    return {"Hello": "World"}


@app.get("/similar/{word}")
async def read_item(word: str):
    return {"similar_words": model._most_similar(word)}
