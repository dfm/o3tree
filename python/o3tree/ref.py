import numpy as np


def ravel(tree):
    if isinstance(tree, dict):
        idx = 0
        flat = []
        unravel = {}
        indices = {}
        for k, v in tree.items():
            values, unravel[k] = ravel(v)
            delta = np.size(values)
            indices[k] = slice(idx, idx + delta)
            idx += delta
            flat.append(values)
        return np.concatenate(flat), lambda x: {
            k: unravel[k](x[indices[k]]) for k in tree
        }

    else:
        shape = np.shape(tree)
        return np.reshape(tree, -1), lambda x: np.reshape(x, shape)
