package tree_sitter_ali_test

import (
	"testing"

	tree_sitter "github.com/smacker/go-tree-sitter"
	"github.com/tree-sitter/tree-sitter-ali"
)

func TestCanLoadGrammar(t *testing.T) {
	language := tree_sitter.NewLanguage(tree_sitter_ali.Language())
	if language == nil {
		t.Errorf("Error loading Ali grammar")
	}
}
