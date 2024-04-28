
def invert_case(text: str):
    opposites = {}
    for (u, l) in zip(iter(text.upper()), iter(text.lower())):
        opposites[u] = l
        opposites[l] = u
    return "".join(opposites[c] for c in iter(text))
