package main

import (
	"archive/zip"
	"bytes"
	"context"
	"fmt"
	"io"
	"os"

	"helm.sh/helm/v3/pkg/action"
	"helm.sh/helm/v3/pkg/chart"
	"helm.sh/helm/v3/pkg/chart/loader"
)

func main() {
	ctx := context.Background()
	zipData, err := io.ReadAll(os.Stdin)
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error reading stdin: %v\n", err)
		os.Exit(1)
	}

	// Create a zip.Reader using the zip data
	zipReader, err := zip.NewReader(bytes.NewReader(zipData), int64(len(zipData)))
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error creating zip reader: %v\n", err)
		os.Exit(1)
	}

	chart := MustLoadChart(ctx, zipReader)
	manifests, err := Template(ctx, TemplateConfig{
		ReleaseName: "my-release",
		Chart:       chart,
		Values:      map[string]any{},
		Namespace:   "default",
	})
	fmt.Printf("manifests: %s, err: %v\n", manifests, err)
}

type Chart struct {
	c *chart.Chart
}

type TemplateConfig struct {
	// ReleaseName is the name of the release
	ReleaseName string

	// Chart is the loaded chart (use `LoadChart`)
	Chart *Chart

	// Values are the configuration variables for the template
	Values map[string]any

	// Namespace defines the namespace oerride
	Namespace string
}

func Template(ctx context.Context, config TemplateConfig) (manifests string, err error) {
	if config.Values == nil {
		config.Values = map[string]any{}
	}

	client := action.NewInstall(&action.Configuration{})
	client.ClientOnly = true
	client.DryRun = true
	client.ReleaseName = config.ReleaseName
	client.IncludeCRDs = true
	client.Namespace = config.Namespace
	rel, err := client.Run(config.Chart.c, config.Values)
	if err != nil {
		return "", fmt.Errorf("unable to render helm chart: %w", err)
	}

	return rel.Manifest, nil
}

// LoadChart loads a chart from a fs.FS system.
// There chart files must be at the root of the provided fs.FS.
// e.g: ./Chart.yaml, ./values.yaml ./templates/deployment.yaml...
func LoadChart(ctx context.Context, z *zip.Reader) (*Chart, error) {
	files := []*loader.BufferedFile{}

	for _, f := range z.File {
		rc, err := f.Open()
		if err != nil {
			return nil, fmt.Errorf("unable to open zip file %s: %w", f.Name, err)
		}
		defer rc.Close()

		data, err := io.ReadAll(rc)
		if err != nil {
			return nil, fmt.Errorf("unable to read zip file %s: %w", f.Name, err)
		}

		files = append(files, &loader.BufferedFile{
			Name: f.Name,
			Data: data,
		})
	}

	chart, err := loader.LoadFiles(files)
	if err != nil {
		return nil, fmt.Errorf("unable to load chart from files: %w", err)
	}

	return &Chart{c: chart}, nil
}

// MustLoadChart is the same as LoadChart but panics if there is
// any error while loading the chart.
func MustLoadChart(ctx context.Context, z *zip.Reader) *Chart {
	chart, err := LoadChart(ctx, z)
	if err != nil {
		panic(err)
	}

	return chart
}
